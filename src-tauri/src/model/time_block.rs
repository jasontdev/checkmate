use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

use crate::model::traits::Entity;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeBlock {
    pub id: i64,
    pub task_id: i64,
    pub start: String,
    pub finish: String,
}

impl TimeBlock {
    fn create(&self, connection: &Connection) -> Result<Self, Error> {
        let mut stmt = connection.prepare("INSERT INTO time_block (task_id, start) VALUES (?1, ?2)")?;
        stmt.execute((&self.task_id, &self.start.to_string()))?;

        Ok(TimeBlock {
            id: connection.last_insert_rowid(),
            task_id: self.task_id,
            start: self.start.to_string(),
            finish: "".to_string(),
        })
    }
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS time_block (id INTEGER PRIMARY KEY, task_id INTEGER, start TEXT, finish TEXT)",
            (),
        )?;
        Ok(())
    }
}

impl Entity for TimeBlock {}
