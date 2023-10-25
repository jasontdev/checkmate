use app::task::{NewTask, Task};
use app::time_block::TimeBlock;
use rusqlite::{Connection, Error};

#[test]
fn find_all_by_date_test() -> Result<(), Error> {
    let connection = Connection::open_in_memory()?;

    Task::create_table(&connection)?;
    TimeBlock::create_table(&connection)?;

    connection.execute(
        "INSERT INTO task (id, description) VALUES (?1, ?2)",
        (1, "Test task 1"),
    )?;
    connection.execute(
        "INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))",
        (1, 1, "2023-09-16T00:00:00+00:00"),
    )?;
    connection.execute(
        "INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))",
        (2, 1, "2023-09-16T12:00:00"),
    )?;
    connection.execute(
        "INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))",
        (3, 1, "2023-09-16T10:00:00+10:00"),
    )?;
    connection.execute(
        "INSERT INTO time_block (id, task_id, start) VALUES (?1, ?2, datetime(?3))",
        (4, 1, "2023-09-15T23:54:29+00:00"),
    )?;

    let tasks = Task::find_all_by_date(&connection, "2023-09-16T00:00:00+00:00".to_string())?;

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks.get(0).unwrap().time_blocks.len(), 3);
    assert_eq!(
        tasks.get(0).unwrap().time_blocks.get(2).unwrap().start,
        Some("2023-09-16 00:00:00".to_string())
    );

    Ok(())
}

#[test]
fn delete() -> Result<(), Error> {
    let connection = Connection::open_in_memory()?;

    Task::create_table(&connection)?;
    TimeBlock::create_table(&connection)?;

    let task = Task {
        id: 1,
        description: "Test task 1".to_string(),
        time_blocks: vec![],
    };

    connection.execute(
        "INSERT INTO task (id, description) VALUES (?1, ?2)",
        (&task.id, &task.description),
    )?;
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

    let tasks = task_count_stmt.query_row([], |row| Ok(Count { i: row.get(0)? }))?;
    let time_blocks = time_block_count_stmt.query_row([], |row| Ok(Count { i: row.get(0)? }))?;

    assert_eq!(tasks.i, 0);
    assert_eq!(time_blocks.i, 0);

    Ok(())
}

#[test]
fn create() -> Result<(), Error> {
    let connection = Connection::open_in_memory()?;

    Task::create_table(&connection)?;
    TimeBlock::create_table(&connection)?;

    let test_string = "Feed the cat";
    Task::create(
        &connection,
        &NewTask {
            description: test_string.to_string(),
        },
    )?;

    let mut task_stmt = connection.prepare(
        format!(
            "SELECT * FROM task WHERE task.description=\"{}\"",
            test_string
        )
        .as_str(),
    )?;

    let row = task_stmt
        .query_row([], |row| row.get::<usize, String>(1))
        .unwrap();

    assert_eq!(row, test_string.to_string());

    return Ok(());
}
