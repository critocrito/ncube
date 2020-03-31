use async_trait::async_trait;
use sqlx::{
    self,
    error::Error as SqlxError,
    sqlite::{SqlitePool as SqlxSqlitePool, SqliteQueryAs},
};

use crate::errors::DataStoreError;
use crate::pools::Pool;

// FIXME: Handle error cases and reasons
impl From<SqlxError> for DataStoreError {
    fn from(_: SqlxError) -> DataStoreError {
        DataStoreError::FailedConnection
    }
}

pub struct SqlitePool {
    pool: SqlxSqlitePool,
}

impl SqlitePool {
    pub async fn new(db_path: &String) -> Result<Self, DataStoreError> {
        let conn_str = format!("sqlite://{}", db_path);
        let pool = SqlxSqlitePool::new(&conn_str).await?;
        Ok(SqlitePool { pool })
    }
}

#[async_trait]
impl Pool for SqlitePool {
    async fn exec(&self) -> Result<i64, DataStoreError> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}
