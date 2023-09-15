use rusqlite::{Connection, Error};

pub trait Entity: Sized {
    fn create(&self, connection: &Connection) -> Result<Self, Error>;
}
