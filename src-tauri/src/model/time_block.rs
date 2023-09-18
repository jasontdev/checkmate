use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

use crate::model::NewTimeBlock;
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
    fn create(connection: &Connection, new_time_block: &NewTimeBlock) -> Result<Self, Error> {
        let mut stmt = connection.prepare("INSERT INTO time_block (task_id, start) VALUES (?1, ?2)")?;
        stmt.execute((&new_time_block.task_id, &new_time_block.start.to_string()))?;

        Ok(TimeBlock {
            id: connection.last_insert_rowid(),
            task_id: new_time_block.task_id,
            start: new_time_block.start.to_string(),
            finish: "".to_string(),
        })
    }
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS time_block (id INTEGER PRIMARY KEY, task_id INTEGER, start TEXT, finish TEXT, FOREIGN KEY(task_id) REFERENCES task(id))",
            (),
        )?;
        Ok(())
    }
}

impl Entity for TimeBlock {}
