use rusqlite::{Connection, Error};
use task::Task;
use time_block::TimeBlock;

pub mod task;
pub mod time_block;

pub fn create_tables(connection: &Connection) -> Result<(), Error> {
    Task::create_table(connection)?;
    TimeBlock::create_table(connection)
}
