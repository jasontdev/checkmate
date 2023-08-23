use rusqlite::{Connection, Error};

pub trait Model {
    fn create_table(&self, connection: &Connection) -> Result<(), Error>;
    fn drop_table(&self, connection: &Connection) -> Result<(), Error>;
}

pub trait Entity: Sized {
    fn create(&self, connection: &Connection) -> Result<Self, Error>;
}
