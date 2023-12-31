use crate::model::traits::Model;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Model for Category {
    fn create_table(&self, connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE function IF NOT EXISTS(
            id INTEGER PRIMARY KEY,
            name TEXT,
            project_id INTEGER
            FOREIGN KEY(project_id) REFERENCES project(id))",
            (),
        )?;
        Ok(())
    }

    fn drop_table(&self, connection: &Connection) -> Result<(), Error> {
        // TODO: will break activity table
        connection.execute("DROP TABLE task", ())?;
        Ok(())
    }
}
