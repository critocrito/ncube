use async_trait::async_trait;
use chrono::Utc;
use rusqlite::{self, params};

use crate::db::{sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn account_store(wrapped_db: Database) -> impl AccountStore {
    match wrapped_db {
        Database::Sqlite(db) => AccountStoreSqlite { db },
    }
}

#[async_trait]
pub(crate) trait AccountStore {
    async fn exists(&self, email: &str, workspace_id: i32) -> Result<bool, StoreError>;
    async fn create(
        &self,
        email: &str,
        password: &str,
        name: Option<String>,
        workspace_id: i32,
    ) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct AccountStoreSqlite {
    db: sqlite::Database,
}

#[async_trait]
impl AccountStore for AccountStoreSqlite {
    async fn exists(&self, email: &str, workspace_id: i32) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/exists.sql"))?;

        let result: i32 = stmt.query_row(params![workspace_id, &email], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    async fn create(
        &self,
        email: &str,
        password: &str,
        name: Option<String>,
        workspace_id: i32,
    ) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/create.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/account/grant_access.sql"))?;

        conn.execute_batch("BEGIN;")?;
        let account_id = stmt.insert(params![
            &email,
            &password,
            &name,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        stmt2.insert(params![
            workspace_id,
            account_id,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;
        conn.execute_batch("COMMIT;")?;

        Ok(())
    }
}
