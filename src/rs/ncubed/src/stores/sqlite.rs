use async_trait::async_trait;
use ncube_data::{Collection, NcubeConfig};
use r2d2::{self, Pool};
use r2d2_sqlite::SqliteConnectionManager;
use refinery_migrations;
use rusqlite::{self, params, Connection};

use crate::errors::DataStoreError;
use crate::stores::NcubeStore;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

// FIXME: Handle error cases and reasons
impl From<r2d2::Error> for DataStoreError {
    fn from(_: r2d2::Error) -> DataStoreError {
        DataStoreError::FailedConnection
    }
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
    pool: Pool<SqliteConnectionManager>,
}

impl NcubeStoreSqlite {
    pub async fn new(db_path: String) -> Result<Self, DataStoreError> {
        let manager = SqliteConnectionManager::file(&db_path);
        let pool = r2d2::Pool::new(manager)?;

        Ok(NcubeStoreSqlite { db_path, pool })
    }
}

#[async_trait]
impl NcubeStore for NcubeStoreSqlite {
    fn upgrade(&mut self) -> Result<(), DataStoreError> {
        let mut conn = Connection::open(&self.db_path)?;
        embedded::migrations::runner().run(&mut conn)?;
        Ok(())
    }

    async fn list_collections(&mut self) -> Result<Vec<Collection>, DataStoreError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(include_str!("../sql/sqlite/list_collections.sql"))?;

        let collections_iter = stmt.query_map(params![], |row| {
            Ok(Collection {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?;

        let mut collections: Vec<Collection> = vec![];
        for collection in collections_iter {
            collections.push(collection.unwrap());
        }

        Ok(collections)
    }

    async fn is_bootstrapped(&mut self) -> Result<bool, DataStoreError> {
        let conn = self.pool.get()?;
        let result: i32 = conn.query_row(
            include_str!("../sql/sqlite/is_bootstrapped.sql"),
            params![],
            |row| row.get(0),
        )?;

        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
