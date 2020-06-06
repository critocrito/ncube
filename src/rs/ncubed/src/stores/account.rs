use async_trait::async_trait;
use chrono::Utc;
use ncube_data::Account;
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, from_rows};

use crate::db::{sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn account_store(wrapped_db: Database) -> impl AccountStore {
    match wrapped_db {
        Database::Sqlite(db) => AccountStoreSqlite { db },
        Database::Http(_client) => todo!(),
    }
}

#[async_trait]
pub(crate) trait AccountStore {
    async fn exists(&self, email: &str, workspace_id: i32) -> Result<bool, StoreError>;
    async fn create(
        &self,
        email: &str,
        password: &str,
        otp: &str,
        name: Option<String>,
        workspace_id: i32,
    ) -> Result<(), StoreError>;
    async fn list(&self) -> Result<Vec<Account>, StoreError>;
    async fn show_password(
        &self,
        email: &str,
        workspace_id: i32,
    ) -> Result<Option<String>, StoreError>;
    async fn update_password(
        &self,
        email: &str,
        hash: &str,
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
        otp: &str,
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
            &otp,
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

    async fn list(&self) -> Result<Vec<Account>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/list.sql"))?;

        let accounts_iter = from_rows::<Account>(stmt.query(NO_PARAMS)?);

        let mut accounts: Vec<Account> = vec![];
        for account in accounts_iter {
            accounts.push(account?);
        }

        Ok(accounts)
    }

    async fn show_password(
        &self,
        email: &str,
        workspace_id: i32,
    ) -> Result<Option<String>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show_password.sql"))?;

        let hash = stmt
            .query_row(params![&email, workspace_id], |row| row.get(0))
            .ok();

        Ok(hash)
    }

    async fn update_password(
        &self,
        email: &str,
        hash: &str,
        workspace_id: i32,
    ) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/update_password.sql"))?;

        stmt.execute(params![&hash, &email, &workspace_id])?;

        Ok(())
    }
}
