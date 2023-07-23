use rusqlite::{Connection, Error};

pub trait Model {
    fn create_table(&self, connection: &Connection) -> Result<(), Error>;
    fn drop_table(&self, connection: &Connection) -> Result<(), Error>;
}

pub trait CRUD: Sized {
    fn create(&self, connection: &Connection) -> Result<Self, Error>;
}
