use refinery_migrations;
use rusqlite::{self, Connection};

use crate::errors::DataStoreError;
use crate::stores::NcubeStore;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

// FIXME: Handle error cases and reasons
impl From<rusqlite::Error> for DataStoreError {
    fn from(_: rusqlite::Error) -> DataStoreError {
        DataStoreError::FailedConnection
    }
}

// FIXME: Handle error cases and reasons
impl From<refinery_migrations::Error> for DataStoreError {
    fn from(_: refinery_migrations::Error) -> DataStoreError {
        DataStoreError::Upgrade
    }
}

pub struct NcubeStoreSqlite {
    conn: Connection,
}

impl NcubeStoreSqlite {
    pub fn new(path: String) -> Result<Self, DataStoreError> {
        let conn = Connection::open(path)?;
        Ok(NcubeStoreSqlite { conn })
    }
}

impl NcubeStore for NcubeStoreSqlite {
    fn upgrade(&mut self) -> Result<(), DataStoreError> {
        embedded::migrations::runner().run(&mut self.conn)?;
        Ok(())
    }
}
