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
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, StoreError>;
    async fn sources_types(&self) -> Result<Stat, StoreError>;
    async fn data_total(&self, query: Option<String>) -> Result<Stat, StoreError>;
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
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_sources.sql"))?;
        let mut stmt2 =
            conn.prepare_cached(include_str!("../sql/stat/count_sources_search.sql"))?;

        let count_sources: i32 = match query {
            Some(q) => stmt2.query_row(params![&q], |row| row.get(0))?,
            None => stmt.query_row(NO_PARAMS, |row| row.get(0))?,
        };

        Ok(Stat {
            name: "sources_total".into(),
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
    async fn data_total(&self, query: Option<String>) -> Result<Stat, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_units.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/stat/count_units_search.sql"))?;

        let count_sources: i32 = match query {
            Some(q) => stmt2.query_row(params![&q], |row| row.get(0))?,
            None => stmt.query_row(NO_PARAMS, |row| row.get(0))?,
        };

        Ok(Stat {
            name: "data_total".into(),
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
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/sources/total",
            self.client.workspace.slug
        ));

        if let Some(q) = query {
            url.query_pairs_mut().clear().append_pair("q", &q);
        }

        let data: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "sources_total".into(),
            value: data,
        })
    }

    #[instrument]
    async fn sources_types(&self) -> Result<Stat, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/sources/types",
            self.client.workspace.slug
        ));

        let data: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "sources_types".into(),
            value: data,
        })
    }

    #[instrument]
    async fn data_total(&self, query: Option<String>) -> Result<Stat, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/total",
            self.client.workspace.slug
        ));

        if let Some(q) = query {
            url.query_pairs_mut().clear().append_pair("q", &q);
        }

        let data: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "data_total".into(),
            value: data,
        })
    }

    #[instrument]
    async fn data_sources(&self) -> Result<Stat, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/sources",
            self.client.workspace.slug
        ));

        let data: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "data_sources".into(),
            value: data,
        })
    }

    #[instrument]
    async fn data_videos(&self) -> Result<Stat, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/videos",
            self.client.workspace.slug
        ));

        let data: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "data_videos".into(),
            value: data,
        })
    }
}
