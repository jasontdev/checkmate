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

    pub fn find_or_create(connection: &Connection, date: String) -> Result<Day, Error> {
        let mut statement = connection.prepare("SELECT id, date FROM day WHERE date=(?)")?;
        let day = statement.query_row([&date], |row| {
            Ok(Day {
                id: row.get(0)?,
                date: row.get(1)?,
                tasks: vec![],
            })
        });

        match day {
            Ok(day) => Ok(day),
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
        let mut statement =
            connection.prepare("INSERT INTO day (date) VALUES (?1) RETURNING id")?;
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
