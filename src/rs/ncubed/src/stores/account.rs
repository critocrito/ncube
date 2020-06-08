use async_trait::async_trait;
use chrono::{Duration, Utc};
use ncube_data::Account;
use rusqlite::{self, params, NO_PARAMS};
use secstr::SecVec;
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};

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
        key: SecVec<u8>,
        name: Option<String>,
        workspace_id: i32,
    ) -> Result<(), StoreError>;
    async fn list(&self) -> Result<Vec<Account>, StoreError>;
    async fn show_password(
        &self,
        email: &str,
        workspace_id: i32,
    ) -> Result<Option<String>, StoreError>;
    async fn show_key(&self, email: &str, workspace_id: i32) -> Result<SecVec<u8>, StoreError>;
    async fn update_password(
        &self,
        email: &str,
        hash: &str,
        key: &SecVec<u8>,
        workspace_id: i32,
    ) -> Result<(), StoreError>;
    async fn show(&self, email: &str, workspace_id: i32) -> Result<Account, StoreError>;
}

#[derive(Debug)]
pub struct AccountStoreSqlite {
    db: Box<sqlite::Database>,
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
        key: SecVec<u8>,
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
            &key.unsecure(),
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

    async fn show_key(&self, email: &str, workspace_id: i32) -> Result<SecVec<u8>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show_key.sql"))?;

        let key = stmt
            .query_row(params![&email, workspace_id], |row| row.get(0))
            .ok()
            .ok_or_else(|| StoreError::NotFound("couldn't retrieve symmetric key".into()))?;

        Ok(SecVec::new(key))
    }

    async fn update_password(
        &self,
        email: &str,
        hash: &str,
        key: &SecVec<u8>,
        workspace_id: i32,
    ) -> Result<(), StoreError> {
        let now = Utc::now();
        let otp_max_age = now - Duration::hours(48);

        let account = self.show(&email, workspace_id).await?;

        if account.is_otp && account.updated_at <= otp_max_age {
            return Err(StoreError::Unauthorized);
        }

        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/update_password.sql"))?;

        stmt.execute(params![
            &hash,
            &key.unsecure(),
            &now.to_rfc3339(),
            &email,
            &workspace_id
        ])?;

        Ok(())
    }

    async fn show(&self, email: &str, workspace_id: i32) -> Result<Account, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&email, workspace_id], |row| {
            from_row_with_columns::<Account>(row, &columns)
        })?;

        let mut accounts: Vec<Account> = vec![];
        for row in rows {
            accounts.push(row?)
        }

        match accounts.first() {
            Some(account) => Ok(account.to_owned()),
            _ => Err(StoreError::NotFound(format!(
                "Account {}/{}",
                email, workspace_id
            ))),
        }
    }
}
