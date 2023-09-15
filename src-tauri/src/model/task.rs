use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub description: String,
}

impl Task {
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS task(id INTEGER PRIMARY KEY, description TEXT)",
            (),
        )?;
        Ok(())
    }
}
