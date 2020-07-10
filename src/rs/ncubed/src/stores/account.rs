use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Account, UpdatePasswordRequest, Workspace};
use rusqlite::{self, params, NO_PARAMS};
use secstr::SecVec;
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};
use tracing::{debug, instrument};

use crate::crypto;
use crate::db::{http, sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn account_store(wrapped_db: Database) -> Box<dyn AccountStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(AccountStoreSqlite { db }),
        Database::Http(client) => Box::new(AccountStoreHttp { client }),
    }
}

#[async_trait]
pub(crate) trait AccountStore {
    async fn exists(&self, email: &str, workspace: &Workspace) -> Result<bool, StoreError>;
    async fn create(
        &self,
        email: &str,
        otp: Option<String>,
        workspace: &Workspace,
    ) -> Result<(), StoreError>;
    async fn list(&self) -> Result<Vec<Account>, StoreError>;
    async fn show_password(&self, email: &str, workspace: &Workspace)
        -> Result<String, StoreError>;
    async fn show_key(&self, email: &str, workspace: &Workspace) -> Result<SecVec<u8>, StoreError>;
    async fn update_password(
        &self,
        email: &str,
        password: &str,
        workspace: &Workspace,
    ) -> Result<String, StoreError>;
    async fn show(&self, email: &str, workspace: &Workspace) -> Result<Account, StoreError>;
    async fn update_hashed_password(
        &self,
        email: &str,
        hash: &str,
        workspace: &Workspace,
    ) -> Result<(), StoreError>;
    async fn show_by_workspace(&self, workspace: &Workspace) -> Result<Account, StoreError>;
}

#[derive(Debug)]
pub struct AccountStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl AccountStore for AccountStoreSqlite {
    #[instrument]
    async fn exists(&self, email: &str, workspace: &Workspace) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/exists.sql"))?;

        let result: i32 = stmt.query_row(params![workspace.id, &email], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
    async fn create(
        &self,
        email: &str,
        otp: Option<String>,
        workspace: &Workspace,
    ) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/create.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/account/grant_access.sql"))?;

        let (hash, otp, key) = match otp {
            Some(otp) => (otp, None, None),
            None => {
                let password = crypto::gen_secret_key(rand::thread_rng());
                let key = crypto::gen_symmetric_key(rand::thread_rng());
                let otp =
                    crypto::aes_encrypt(rand::thread_rng(), &key, &password.as_bytes().to_vec());
                let hash = crypto::hash(rand::thread_rng(), password.as_bytes());

                (hash, Some(otp), Some(key))
            }
        };

        conn.execute_batch("BEGIN;")?;
        let account_id = stmt.insert(params![
            &email,
            &hash,
            &otp,
            &key.map(|v| v.unsecure().to_owned()),
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        stmt2.insert(params![
            workspace.id,
            account_id,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;
        conn.execute_batch("COMMIT;")?;

        Ok(())
    }

    #[instrument]
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

    #[instrument]
    async fn show_password(
        &self,
        email: &str,
        workspace: &Workspace,
    ) -> Result<String, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show_password.sql"))?;

        let hash: String = stmt
            .query_row(params![&email, workspace.id], |row| row.get(0))
            .map_err(|_| StoreError::NotFound("couldn't retrieve password".into()))?;

        Ok(hash.to_string())
    }

    #[instrument]
    async fn show_key(&self, email: &str, workspace: &Workspace) -> Result<SecVec<u8>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show_key.sql"))?;

        let key = stmt
            .query_row(params![&email, workspace.id], |row| row.get(0))
            .ok()
            .ok_or_else(|| StoreError::NotFound("couldn't retrieve symmetric key".into()))?;

        Ok(SecVec::new(key))
    }

    #[instrument]
    async fn update_password(
        &self,
        email: &str,
        password: &str,
        workspace: &Workspace,
    ) -> Result<String, StoreError> {
        let now = Utc::now();

        let hash = crypto::hash(rand::thread_rng(), password.as_bytes());
        let key = crypto::gen_symmetric_key(rand::thread_rng());

        debug!("update password {} with hash {}", password, hash);

        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/update_password.sql"))?;

        stmt.execute(params![
            &hash,
            &key.unsecure(),
            &now.to_rfc3339(),
            &email,
            workspace.id
        ])?;

        let new_password =
            crypto::aes_encrypt(rand::thread_rng(), &key, &password.as_bytes().to_vec());

        Ok(new_password)
    }

    #[instrument]
    async fn show(&self, email: &str, workspace: &Workspace) -> Result<Account, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&email, workspace.id], |row| {
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
                email, workspace.id
            ))),
        }
    }

    #[instrument]
    async fn update_hashed_password(
        &self,
        email: &str,
        hash: &str,
        workspace: &Workspace,
    ) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/update_hash.sql"))?;

        stmt.execute(params![&hash, &email, workspace.id])?;

        Ok(())
    }

    #[instrument]
    async fn show_by_workspace(&self, workspace: &Workspace) -> Result<Account, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/account/show_by_workspace.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&workspace.id], |row| {
            from_row_with_columns::<Account>(row, &columns)
        })?;

        let mut accounts: Vec<Account> = vec![];
        for row in rows {
            accounts.push(row?)
        }

        match accounts.first() {
            Some(account) => Ok(account.to_owned()),
            _ => Err(StoreError::NotFound(format!(
                "no account associated to workspace {}",
                workspace.slug
            ))),
        }
    }
}

#[derive(Debug)]
pub struct AccountStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl AccountStore for AccountStoreHttp {
    #[instrument]
    async fn exists(&self, _email: &str, _workspace: &Workspace) -> Result<bool, StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn create(
        &self,
        _email: &str,
        _otp: Option<String>,
        _workspace: &Workspace,
    ) -> Result<(), StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Account>, StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn show_password(
        &self,
        _email: &str,
        _workspace: &Workspace,
    ) -> Result<String, StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn show_key(
        &self,
        _email: &str,
        _workspace: &Workspace,
    ) -> Result<SecVec<u8>, StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn update_password(
        &self,
        email: &str,
        password: &str,
        _workspace: &Workspace,
    ) -> Result<String, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/account",
            self.client.workspace.slug
        ));

        let payload = UpdatePasswordRequest {
            email: email.to_string(),
            password: password.to_string(),
            password_again: password.to_string(),
        };

        let new_password: String = self.client.put(url, &payload).await?.unwrap();

        Ok(new_password)
    }

    #[instrument]
    async fn show(&self, _email: &str, _workspace: &Workspace) -> Result<Account, StoreError> {
        unreachable!()
    }

    #[instrument]
    async fn update_hashed_password(
        &self,
        _email: &str,
        _hash: &str,
        _workspace: &Workspace,
    ) -> Result<(), StoreError> {
        unreachable!();
    }

    #[instrument]
    async fn show_by_workspace(&self, _workspace: &Workspace) -> Result<Account, StoreError> {
        unreachable!();
    }
}
