use async_trait::async_trait;

use ncube_data::Stat;
use rusqlite::{params, NO_PARAMS};
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
    async fn sources_total(&self) -> Result<Stat, StoreError>;
    async fn sources_total_search(&self, query: &str) -> Result<Stat, StoreError>;
    async fn sources_types(&self) -> Result<Stat, StoreError>;
    async fn data_total(&self) -> Result<Stat, StoreError>;
    async fn data_total_search(&self, query: &str) -> Result<Stat, StoreError>;
    async fn data_sources(&self) -> Result<Stat, StoreError>;
    async fn data_videos(&self) -> Result<Stat, StoreError>;
}

#[derive(Debug)]
pub struct StatStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl StatStore for StatStoreSqlite {
    #[instrument]
    async fn sources_total(&self) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_sources.sql"))?;

        let count_sources: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "sources_total".into(),
            value: count_sources,
        })
    }

    #[instrument]
    async fn sources_total_search(&self, query: &str) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_sources_search.sql"))?;

        let count_sources: i32 = stmt.query_row(params![&query], |row| row.get(0))?;

        Ok(Stat {
            name: "sources_total_search".into(),
            value: count_sources,
        })
    }

    #[instrument]
    async fn sources_types(&self) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_source_types.sql"))?;

        let count_source_types: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "sources_types".into(),
            value: count_source_types,
        })
    }

    #[instrument]
    async fn data_total(&self) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_units.sql"))?;

        let count_sources: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_total".into(),
            value: count_sources,
        })
    }

    #[instrument]
    async fn data_total_search(&self, query: &str) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_units_search.sql"))?;

        let count_sources: i32 = stmt.query_row(params![&query], |row| row.get(0))?;

        Ok(Stat {
            name: "data_total_search".into(),
            value: count_sources,
        })
    }

    #[instrument]
    async fn data_sources(&self) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_unit_types.sql"))?;

        let count_unit_types: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_sources".into(),
            value: count_unit_types,
        })
    }

    #[instrument]
    async fn data_videos(&self) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_videos.sql"))?;

        let count_videos: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_videos".into(),
            value: count_videos,
        })
    }
}

#[derive(Debug)]
pub struct StatStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl StatStore for StatStoreHttp {
    #[instrument]
    async fn sources_total(&self) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn sources_types(&self) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn sources_total_search(&self, _query: &str) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn data_total(&self) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn data_total_search(&self, _query: &str) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn data_sources(&self) -> Result<Stat, StoreError> {
        todo!()
    }

    #[instrument]
    async fn data_videos(&self) -> Result<Stat, StoreError> {
        todo!()
    }
}
