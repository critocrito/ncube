use async_trait::async_trait;
use ncube_data::{ConfigSetting, NcubeConfig};
use ncube_db::{errors::DatabaseError, sqlite, Database};
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, from_rows};
use std::fmt::Debug;
use tracing::instrument;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub fn config_store(wrapped_db: Database) -> impl ConfigStore {
    match wrapped_db {
        Database::Sqlite(db) => ConfigStoreSqlite { db },
        Database::Http(_client) => todo!(),
    }
}

#[async_trait]
pub trait ConfigStore {
    async fn init(&self) -> Result<(), DatabaseError>;
    async fn upgrade(&self) -> Result<(), DatabaseError>;
    async fn is_bootstrapped(&self) -> Result<bool, DatabaseError>;
    async fn show(&self) -> Result<NcubeConfig, DatabaseError>;
    async fn show_all(&self) -> Result<NcubeConfig, DatabaseError>;
    async fn insert(&self, name: &str, value: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct ConfigStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl ConfigStore for ConfigStoreSqlite {
    #[instrument]
    async fn init(&self) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        conn.pragma_update(None, "foreign_keys", &"ON")?;
        // FIXME: Should I enable this?
        // conn.pragma_update(None, "journal_mode", &"WAL")?;
        Ok(())
    }

    #[instrument]
    async fn upgrade(&self) -> Result<(), DatabaseError> {
        let mut conn = self.db.connection().await?;
        // The actual sqlite connection is hidden inside a deadpool Object
        // inside a ClientWrapper. We deref those two levels to make refinery
        // happy.
        embedded::migrations::runner().run(&mut **conn)?;
        Ok(())
    }

    #[instrument]
    async fn is_bootstrapped(&self) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let result: i32 = conn.query_row(
            include_str!("../sql/config/is_bootstrapped.sql"),
            NO_PARAMS,
            |row| row.get(0),
        )?;

        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    #[instrument]
    async fn show(&self) -> Result<NcubeConfig, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare(include_str!("../sql/config/show.sql"))?;

        let config_iter = from_rows::<ConfigSetting>(stmt.query(NO_PARAMS)?);

        let mut ncube_config: NcubeConfig = vec![];
        for setting in config_iter {
            ncube_config.push(setting?);
        }

        Ok(ncube_config)
    }

    #[instrument]
    async fn show_all(&self) -> Result<NcubeConfig, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare(include_str!("../sql/config/show_all.sql"))?;

        let config_iter = from_rows::<ConfigSetting>(stmt.query(NO_PARAMS)?);

        let mut ncube_config: NcubeConfig = vec![];
        for setting in config_iter {
            ncube_config.push(setting?);
        }

        Ok(ncube_config)
    }

    #[instrument]
    async fn insert(&self, name: &str, value: &str) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let setting_id: i32 = conn.query_row(
            include_str!("../sql/config/setting_exists.sql"),
            params![&name],
            |row| row.get(0),
        )?;

        conn.execute(
            include_str!("../sql/config/upsert.sql"),
            params![&setting_id, &value],
        )?;

        Ok(())
    }
}
