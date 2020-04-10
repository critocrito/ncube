use async_trait::async_trait;
use ncube_data::{Collection, ConfigSetting, NcubeConfig};
use r2d2::{self, Pool};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{self, params, Connection, NO_PARAMS};
use serde_rusqlite::{self, from_rows};

use crate::errors::DataStoreError;
use crate::stores::NcubeStore;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
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

        let collections_iter = from_rows::<Collection>(stmt.query(NO_PARAMS)?);

        let mut collections: Vec<Collection> = vec![];
        for collection in collections_iter {
            collections.push(collection?);
        }

        Ok(collections)
    }

    async fn is_bootstrapped(&mut self) -> Result<bool, DataStoreError> {
        let conn = self.pool.get()?;
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

    async fn show(&mut self) -> Result<NcubeConfig, DataStoreError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(include_str!("../sql/sqlite/show_ncube_config.sql"))?;

        let config_iter = from_rows::<ConfigSetting>(stmt.query(NO_PARAMS)?);

        let mut ncube_config: NcubeConfig = vec![];
        for setting in config_iter {
            ncube_config.push(setting?);
        }

        Ok(ncube_config)
    }

    async fn insert(&mut self, name: &String, value: &String) -> Result<(), DataStoreError> {
        let conn = self.pool.get()?;
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
