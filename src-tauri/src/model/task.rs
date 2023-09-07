use crate::model::category::Category;
use crate::model::project::Project;
use crate::model::traits::Entity;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub day_id: i64,
    pub description: String,
    pub project: Option<Project>,
    pub category: Option<Category>,
}

impl Task {
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS task(id INTEGER PRIMARY KEY, day_id INTEGER, description TEXT)",
            (),
        )?;
        Ok(())
    }

    pub fn find_all_by_date(connection: &Connection, date: String) -> Result<Vec<Task>, Error> {
        let mut statement = connection.prepare("SELECT task.id, task.day_id, task.description FROM day INNER JOIN task ON task.day_id=day.id WHERE day.date=(?)")?;
        let mut rows = statement.query([&date])?;

        let mut tasks = Vec::new();
        while let Some(row) = rows.next()? {
            tasks.push(Task {
                id: row.get(0)?,
                day_id: row.get(1)?,
                description: row.get(2)?,
                project: None,
                category: None,
            })
        }

        Ok(tasks)
    }
}

impl Entity for Task {
    fn create(&self, connection: &Connection) -> Result<Self, Error> {
        let mut statement =
            connection.prepare("INSERT INTO task (day_id, description) VALUES (?1, ?2)")?;
        statement.execute((&self.day_id, &self.description))?;

        Ok(Task {
            id: connection.last_insert_rowid(),
            day_id: self.day_id,
            description: self.description.to_string(),
            project: None,
            category: None,
        })
    }
}
