use std::collections::HashMap;

use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

use crate::model::{NewTask, TimeBlock};
use crate::model::traits::Entity;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub time_blocks: Vec<TimeBlock>,
}

impl Task {
    pub fn create_table(connection: &Connection) -> Result<(), Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS task(id INTEGER PRIMARY KEY, date TEXT, description TEXT)",
            (),
        )?;
        Ok(())
    }

    fn create(connection: &Connection, new_task: &NewTask) -> Result<Self, Error> {
        let mut stmt = connection.prepare("INSERT
        INTO
        task(description)
        VALUES(?1)
        ")?;
        stmt.execute([&new_task.description])?;

        Ok(Task {
            id: connection.last_insert_rowid(),
            description: new_task.description.to_string(),
            time_blocks: vec![],
        })
    }

    pub fn find_all_by_date(connection: &Connection, date: String) -> Result<Vec<Task>, Error> {
        let mut stmt =
            connection.prepare("SELECT task.id, task.description, time_block.id AS time_block_id, \
            time_block.start, time_block.finish FROM task JOIN time_block ON DATE(time_block.start)=?1")?;
        let task_time_block_iter = stmt.query_map([date], |row| {
            Ok((Task {
                id: row.get(0)?,
                description: row.get(1)?,
                time_blocks: vec![],
            }, TimeBlock {
                id: row.get(2)?,
                task_id: row.get(0)?,
                start: row.get(3)?,
                finish: row.get(4)?,
            }))
        })?;

        let mut tasks = HashMap::new();
        for task_time_block in task_time_block_iter {
            let task_time_block_unwrapped = task_time_block.unwrap();
            let task = task_time_block_unwrapped.0;
            let time_block = task_time_block_unwrapped.1;
            if !tasks.contains_key(&task.id) {
                tasks.insert(task.id, task.clone());
            }

            let x = tasks.get_mut(&task.id);
            x.unwrap().time_blocks.push(time_block);
        }

        Ok(tasks.iter().map(|(_i, mapped_task)| mapped_task.clone()).collect())
    }
}

impl Entity for Task {}

mod tests {
    use rusqlite::{Connection, Error};

    use crate::model::{Task, TimeBlock};

    #[test]
    fn find_all_by_date_test() -> Result<(), Error> {
        let connection = Connection::open_in_memory().expect("DB connection error:");

        Task::create_table(&connection)?;
        TimeBlock::create_table(&connection)?;

        connection.execute("INSERT INTO task (id, description) VALUES (?1, ?2)", (1, "Test task 1"))?;
        connection.execute("INSERT INTO time_block (id, task_id, start, finish) VALUES (?1, ?2, ?3, ?4)", (1, 1, "2023-09-16T23:54:29+00:00", ""))?;
        connection.execute("INSERT INTO time_block (id, task_id, start, finish) VALUES (?1, ?2, ?3, ?4)", (2, 1, "2023-09-16T23:54:29", ""))?;
        connection.execute("INSERT INTO time_block (id, task_id, start, finish) VALUES (?1, ?2, ?3, ?4)", (3, 1, "2023-09-16T23:54:29Z", ""))?;
        connection.execute("INSERT INTO time_block (id, task_id, start, finish) VALUES (?1, ?2, ?3, ?4)", (4, 1, "2023-09-15T23:54:29+00:00", ""))?;

        let tasks = Task::find_all_by_date(&connection, "2023-09-16".to_string())?;

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.get(0).unwrap().time_blocks.len(), 3);
        assert_eq!(tasks.get(0).unwrap().time_blocks.get(0).unwrap().start, Some("2023-09-16T23:54:29+00:00".to_string()));

        Ok(())
    }
}
