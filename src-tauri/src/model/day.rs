use crate::model::task::Task;
use crate::model::traits::{Model, CRUD};
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub id: i32,
    pub date: String,
    pub activities: Vec<Task>,
}

impl Day {
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS day(id INTEGER PRIMARY KEY, date TEXT)",
            (),
        )?;
        Ok(())
    }

    pub fn drop_table(connection: &Connection) -> Result<(), Error> {
        connection.execute("DROP TABLE day", ())?;
        Ok(())
    }

    pub fn find_by_date(connection: &Connection, date: String) -> Result<Option<Day>, Error> {
        let mut statement = connection.prepare("SELECT id, date FROM day WHERE date=(?)")?;
        let day = statement.query_row([&date], |row| {
            Ok(Day {
                id: row.get(0)?,
                date: row.get(1)?,
                activities: vec![],
            })
        });

        match day {
            Ok(day) => Ok(Some(day)),
            Err(_) => {
                let new_day = Day {
                    id: 0,
                    date: date.to_string(),
                    activities: vec![],
                };

                match new_day.create(connection) {
                    Ok(d) => Ok(Some(d)),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl CRUD for Day {
    fn create(&self, connection: &Connection) -> Result<Self, Error> {
        let mut statement =
            connection.prepare("INSERT INTO day (date) VALUES (?1) RETURNING id")?;
        statement.query_row([&self.date], |row| {
            Ok(Day {
                id: row.get(0)?,
                date: self.date.to_string(),
                activities: vec![],
            })
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
