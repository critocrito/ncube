use async_trait::async_trait;

use ncube_data::Stat;
use rusqlite::NO_PARAMS;
use tracing::instrument;

use crate::db::{http, sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn stat_store(wrapped_db: Database) -> Box<dyn StatStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(StatStoreSqlite { db }),
        Database::Http(client) => Box::new(StatStoreHttp { client }),
    }
}

#[async_trait]
pub(crate) trait StatStore {
    async fn sources(&self) -> Result<Vec<Stat>, StoreError>;
    async fn data(&self) -> Result<Vec<Stat>, StoreError>;
}

#[derive(Debug)]
pub struct StatStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl StatStore for StatStoreSqlite {
    #[instrument]
    async fn sources(&self) -> Result<Vec<Stat>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_sources.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/stat/count_source_types.sql"))?;

        let count_sources: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;
        let count_source_types: i32 = stmt2.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(vec![
            Stat {
                name: "count_sources".into(),
                value: count_sources,
            },
            Stat {
                name: "count_source_types".into(),
                value: count_source_types,
            },
        ])
    }

    #[instrument]
    async fn data(&self) -> Result<Vec<Stat>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_units.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/stat/count_unit_types.sql"))?;

        let count_units: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;
        let count_unit_types: i32 = stmt2.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(vec![
            Stat {
                name: "count_units".into(),
                value: count_units,
            },
            Stat {
                name: "count_unit_types".into(),
                value: count_unit_types,
            },
        ])
    }
}

#[derive(Debug)]
pub struct StatStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl StatStore for StatStoreHttp {
    #[instrument]
    async fn sources(&self) -> Result<Vec<Stat>, StoreError> {
        todo!()
    }

    #[instrument]
    async fn data(&self) -> Result<Vec<Stat>, StoreError> {
        todo!()
    }
}
