use async_trait::async_trait;
use refinery_migrations;
use rusqlite::{self, Connection};

use crate::errors::DataStoreError;
use crate::pools::{sqlite::SqlitePool, Pool};
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
    pool: Box<dyn Pool + Send + Sync>,
}

impl NcubeStoreSqlite {
    pub async fn new(db_path: String) -> Result<Self, DataStoreError> {
        let pool = SqlitePool::new(&db_path.clone().into()).await?;
        Ok(NcubeStoreSqlite {
            db_path,
            pool: Box::new(pool),
        })
    }
}

#[async_trait]
impl NcubeStore for NcubeStoreSqlite {
    fn upgrade(&mut self) -> Result<(), DataStoreError> {
        let mut conn = Connection::open(&self.db_path)?;
        embedded::migrations::runner().run(&mut conn)?;
        Ok(())
    }

    async fn show_number(&mut self) -> Result<i64, DataStoreError> {
        let pool = &self.pool;
        let num1 = pool.exec();
        num1.await
    }
}
