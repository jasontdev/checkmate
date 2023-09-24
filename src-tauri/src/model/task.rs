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
            "CREATE TABLE IF NOT EXISTS task(id INTEGER PRIMARY KEY, description TEXT)",
            (),
        )?;
        Ok(())
    }

    fn create(connection: &Connection, new_task: &NewTask) -> Result<Task, Error> {
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

    fn update(connection: &Connection, task: &Task) -> Result<Task, Error> {
        let mut stmt = connection.prepare("UPDATE task SET description=?1 WHERE task.id=?2")?;
        stmt.execute((&task.description, &task.id))?;

        Ok(Task {
            id: task.id,
            description: "".to_string(),
            time_blocks: task.time_blocks.clone(), // TODO: should be updatable?
        })
    }

    fn delete(connection: &Connection, task: &Task) -> Result<(), Error> {
        connection.execute("DELETE FROM time_block WHERE task_id=?1; DELETE FROM task WHERE id=?1", [&task.id])?;
        connection.execute("DELETE FROM task WHERE id=?1", [&task.id])?;

        Ok(())
    }

    pub fn find_all_by_date(connection: &Connection, date: String) -> Result<Vec<Task>, Error> {
        let mut stmt =
            // connection.prepare("SELECT task.id, task.description, time_block.id AS time_block_id, \
            // time_block.start, time_block.finish FROM task JOIN time_block ON DATE(time_block.start)=?1")?;
            connection.prepare("SELECT task.id, task.description, time_block.id AS time_block_id, \
            datetime(time_block.start), datetime(time_block.finish) FROM task JOIN time_block ON time_block.task_id=task.id AND datetime(time_block.start) BETWEEN datetime(?1) AND datetime(?1, \"+24 hour\")")?;
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
        connection.execute("INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))", (1, 1, "2023-09-16T00:00:00+00:00"))?;
        connection.execute("INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))", (2, 1, "2023-09-16T12:00:00"))?;
        connection.execute("INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))", (3, 1, "2023-09-16T10:00:00+10:00"))?;
        connection.execute("INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))", (4, 1, "2023-09-15T23:54:29+00:00"))?;

        let tasks = Task::find_all_by_date(&connection, "2023-09-16T00:00:00+00:00".to_string())?;

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.get(0).unwrap().time_blocks.len(), 3);
        assert_eq!(tasks.get(0).unwrap().time_blocks.get(2).unwrap().start, Some("2023-09-16 00:00:00".to_string()));

        Ok(())
    }

    #[test]
    fn delete() -> Result<(), Error> {
        let connection = Connection::open_in_memory().expect("DB connection error:");

        Task::create_table(&connection)?;
        TimeBlock::create_table(&connection)?;

        let task = Task {
            id: 1,
            description: "Test task 1".to_string(),
            time_blocks: vec![],
        };

        connection.execute("INSERT INTO task (id, description) VALUES (?1, ?2)", (&task.id, &task.description))?;
        connection.execute("INSERT INTO time_block (task_id) VALUES (?1)", [&task.id])?;
        connection.execute("INSERT INTO time_block (task_id) VALUES (?1)", [&task.id])?;
        connection.execute("INSERT INTO time_block (task_id) VALUES (?1)", [&task.id])?;
        connection.execute("INSERT INTO time_block (task_id) VALUES (?1)", [&task.id])?;

        Task::delete(&connection, &task)?;
        let mut task_count_stmt = connection.prepare("SELECT COUNT(*) FROM task")?;
        let mut time_block_count_stmt = connection.prepare("SELECT COUNT(*) FROM time_block")?;

        struct Count {
            i: i32,
        }

        let tasks = task_count_stmt.query_row([], |row| Ok(Count {
            i: row.get(0)?,
        }))?;
        let time_blocks = time_block_count_stmt.query_row([], |row| Ok(Count {
            i: row.get(0)?,
        }))?;

        assert_eq!(tasks.i, 0);
        assert_eq!(time_blocks.i, 0);

        Ok(())
    }
}
