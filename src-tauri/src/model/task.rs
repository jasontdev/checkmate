use crate::model::function::Function;
use crate::model::project::Project;
use crate::model::traits::CRUD;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub day_id: i64,
    pub description: String,
    pub project: Option<Project>,
    pub task: Option<Function>,
}

impl CRUD for Task {
    fn create(&self, connection: &Connection) -> Result<Self, Error> {
        let mut statement = connection
            .prepare("INSERT INTO task (day_id, description) VALUES (?1, ?2) RETURNING id")?;
        statement.execute((&self.day_id, &self.description))?;

        Ok(Task {
            id: connection.last_insert_rowid(),
            day_id: self.day_id,
            description: self.description.to_string(),
            project: None,
            task: None,
        })
    }
}
