use async_trait::async_trait;
use ncube_data::{Collection, ConfigSetting, NcubeConfig};
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, from_rows};
use std::fmt::{self, Debug};

use crate::db::sqlite;
use crate::errors::StoreError;
use crate::stores::NcubeStore;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub struct NcubeStoreSqlite {
    db: sqlite::Database,
}

impl NcubeStoreSqlite {
    pub async fn new(db_path: String) -> Result<Self, StoreError> {
        let config = db_path.parse::<sqlite::Config>().unwrap();
        let db = sqlite::Database::new(config, 10);

        Ok(NcubeStoreSqlite { db })
    }
}

impl Debug for NcubeStoreSqlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NcubeStoreSqlite")
            .field("config", &self.db.config)
            .finish()
    }
}

#[async_trait]
impl NcubeStore for NcubeStoreSqlite {
    async fn upgrade(&mut self) -> Result<(), StoreError> {
        let mut conn = self.db.connection().await?;
        // The actual sqlite connection is hidden inside a deadpool Object
        // inside a ClientWrapper. We deref those two levels to make refinery
        // happy.
        embedded::migrations::runner().run(&mut **conn)?;
        Ok(())
    }

    async fn list_collections(&mut self) -> Result<Vec<Collection>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare(include_str!("../sql/sqlite/list_collections.sql"))?;

        let collections_iter = from_rows::<Collection>(stmt.query(NO_PARAMS)?);

        let mut collections: Vec<Collection> = vec![];
        for collection in collections_iter {
            collections.push(collection?);
        }

        Ok(collections)
    }

    async fn is_bootstrapped(&mut self) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let result: i32 = conn.query_row(
            include_str!("../sql/sqlite/is_bootstrapped.sql"),
            NO_PARAMS,
            |row| row.get(0),
        )?;

        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn show(&mut self) -> Result<NcubeConfig, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare(include_str!("../sql/sqlite/show_ncube_config.sql"))?;

        let config_iter = from_rows::<ConfigSetting>(stmt.query(NO_PARAMS)?);

        let mut ncube_config: NcubeConfig = vec![];
        for setting in config_iter {
            ncube_config.push(setting?);
        }

        Ok(ncube_config)
    }

    async fn insert(&mut self, name: &str, value: &str) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let setting_id: i32 = conn.query_row(
            include_str!("../sql/sqlite/setting_exists.sql"),
            params![&name],
            |row| row.get(0),
        )?;

        conn.execute(
            include_str!("../sql/sqlite/create_ncube_config.sql"),
            params![&setting_id, &value],
        )?;

        Ok(())
    }
}
