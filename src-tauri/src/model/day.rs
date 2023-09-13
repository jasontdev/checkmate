use crate::model::task::Task;
use crate::model::traits::Entity;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Day {
    pub id: i64,
    pub date: String,
    pub tasks: Vec<Task>,
}

impl Day {
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS day(id INTEGER PRIMARY KEY, date TEXT)",
            (),
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn drop_table(connection: &Connection) -> Result<(), Error> {
        connection.execute("DROP TABLE day", ())?;
        Ok(())
    }

    pub fn find_by_date(connection: &Connection, date: String) -> Result<Day, Error> {
        let mut day_query_statement =
            connection.prepare("SELECT id, date FROM day WHERE date=(?)")?;

        day_query_statement.query_row([&date], |row| {
            Ok(Day {
                id: row.get(0)?,
                date: row.get(1)?,
                tasks: vec![],
            })
        })
    }

    pub fn find_or_create(connection: &Connection, date: String) -> Result<Day, Error> {
        // let mut statement = connection.prepare("SELECT day.id, day.date FROM day INNER JOIN task ON task.day_id = day.id WHERE date=(?)")?;
        // let mut rows = statement.query([&date])?;
        // while let Some(rows) = rows.next()? {
        //     let row = rows.get(0)?;
        //     println!("{}", row);
        // }

        // TODO: remove clone
        let day_query_result = Day::find_by_date(&connection, date.clone());

        match day_query_result {
            Ok(day_query_result) => {
                let tasks = match Task::find_all_by_date(&connection, date.clone()) {
                    Ok(tasks) => tasks,
                    Err(_) => vec![],
                };

                Ok(Day {
                    id: day_query_result.id,
                    date: day_query_result.date,
                    tasks,
                })
            }
            Err(_) => {
                let new_day = Day {
                    id: 0,
                    date: date.to_string(),
                    tasks: vec![],
                };

                match new_day.create(connection) {
                    Ok(d) => Ok(d),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl Entity for Day {
    fn create(&self, connection: &Connection) -> Result<Self, Error> {
        let mut statement = connection.prepare("INSERT INTO day (date) VALUES (?1)")?;
        statement.execute([&self.date])?;

        Ok(Day {
            id: connection.last_insert_rowid(),
            date: self.date.to_string(),
            tasks: vec![],
        })
    }
}

#[cfg(test)]
mod test {
    use crate::model::Day;
    use rusqlite::Connection;

    #[test]
    fn create_table() {
        let connection = Connection::open_in_memory();
        if connection.is_err() {
            panic!("{}", connection.unwrap_err().to_string())
        }

        assert_eq!(Day::create_table(&connection.unwrap()), Ok(()));
    }
}
