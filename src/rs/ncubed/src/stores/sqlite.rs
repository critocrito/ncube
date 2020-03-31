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
    db_path: String,
}

impl NcubeStoreSqlite {
    pub fn new(db_path: String) -> Result<Self, DataStoreError> {
        Ok(NcubeStoreSqlite { db_path })
    }
}

impl NcubeStore for NcubeStoreSqlite {
    fn upgrade(&mut self) -> Result<(), DataStoreError> {
        let mut conn = Connection::open(&self.db_path)?;
        embedded::migrations::runner().run(&mut conn)?;
        Ok(())
    }
}
