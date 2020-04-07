use async_trait::async_trait;
use ncube_data::Collection;
use refinery_migrations;
use rusqlite::{self, Connection};
use sqlx::{self, error::Error as SqlxError, sqlite::SqlitePool as SqlxSqlitePool, Cursor, Row};

use crate::errors::DataStoreError;
use crate::stores::NcubeStore;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

// FIXME: Handle error cases and reasons
impl From<SqlxError> for DataStoreError {
    fn from(_: SqlxError) -> DataStoreError {
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
    pool: SqlxSqlitePool,
}

impl NcubeStoreSqlite {
    pub async fn new(db_path: String) -> Result<Self, DataStoreError> {
        let conn_str = format!("sqlite://{}", db_path);
        let pool = SqlxSqlitePool::new(&conn_str).await?;
        // let pool = SqlxSqlitePool::builder()
        //     .min_size(5)
        //     .max_size(10)
        //     .build(&conn_str)
        //     .await?;

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
        let mut conn = self.pool.acquire().await?;
        let mut cursor =
            sqlx::query(include_str!("../sql/sqlite/list_collections.sql")).fetch(&mut conn);

        let mut collections: Vec<Collection> = vec![];
        while let Some(row) = cursor.next().await? {
            collections.push(Collection {
                id: row.get("id"),
                title: row.get("title"),
            })
        }

        Ok(collections)
    }
}
