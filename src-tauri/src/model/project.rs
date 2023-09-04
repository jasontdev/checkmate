use crate::model::traits::Model;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

impl Model for Project {
    fn create_table(&self, connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE project IF NOT EXISTS(
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE)",
            (),
        )?;

        Ok(())
    }

    fn drop_table(&self, connection: &Connection) -> Result<(), Error> {
        // TODO: will break activity table
        connection.execute("DROP TABLE project", ())?;
        Ok(())
    }
}
