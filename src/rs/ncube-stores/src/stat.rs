use async_trait::async_trait;
use ncube_data::Stat;
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use tracing::instrument;

pub fn stat_store(wrapped_db: Database) -> Box<dyn StatStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(StatStoreSqlite { db }),
        Database::Http(client) => Box::new(StatStoreHttp { client }),
    }
}

#[async_trait]
pub trait StatStore {
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, DatabaseError>;
    async fn sources_types(&self) -> Result<Stat, DatabaseError>;
    async fn data_total(&self, query: Option<String>) -> Result<Stat, DatabaseError>;
    async fn data_sources(&self) -> Result<Stat, DatabaseError>;
    async fn data_videos(&self) -> Result<Stat, DatabaseError>;
    async fn data_segments(&self) -> Result<Stat, DatabaseError>;
    async fn processes_all(&self, process: &str) -> Result<Stat, DatabaseError>;
    async fn investigations_total(&self) -> Result<Stat, DatabaseError>;
    async fn investigation_segments_total(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError>;
}

#[derive(Debug)]
pub struct StatStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl StatStore for StatStoreSqlite {
    #[instrument]
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, DatabaseError> {
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
    async fn sources_types(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_source_types.sql"))?;

        let count_source_types: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "sources_types".into(),
            value: count_source_types,
        })
    }

    #[instrument]
    async fn data_total(&self, query: Option<String>) -> Result<Stat, DatabaseError> {
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
    async fn data_sources(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_unit_types.sql"))?;

        let count_unit_types: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_sources".into(),
            value: count_unit_types,
        })
    }

    #[instrument]
    async fn data_videos(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_videos.sql"))?;

        let count_videos: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_videos".into(),
            value: count_videos,
        })
    }

    #[instrument]
    async fn data_segments(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_segments.sql"))?;

        let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "data_segments".into(),
            value: count,
        })
    }

    #[instrument]
    async fn processes_all(&self, process: &str) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/stat/count_all_sources_process.sql"))?;

        // At first glance it appears as if process keys and source types are
        // the same. But this isn't necessarly true.
        let source_type = match process {
            "youtube_video" => "youtube_video",
            "youtube_channel" => "youtube_channel",
            "twitter_tweet" => "twitter_tweet",
            "twitter_feed" => "twitter_user",
            _ => "http_url",
        };

        let value: i32 = stmt.query_row(params![&source_type], |row| row.get(0))?;

        Ok(Stat {
            name: "processes_all".into(),
            value,
        })
    }

    #[instrument]
    async fn investigations_total(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_investigations.sql"))?;

        let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "investigations_total".into(),
            value: count,
        })
    }

    #[instrument]
    async fn investigation_segments_total(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/stat/count_investigation_segments.sql"))?;

        let count: i32 = stmt.query_row(params![&investigation], |row| row.get(0))?;

        Ok(Stat {
            name: "investigation_segments_total".into(),
            value: count,
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
    async fn sources_total(&self, query: Option<String>) -> Result<Stat, DatabaseError> {
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
    async fn sources_types(&self) -> Result<Stat, DatabaseError> {
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
    async fn data_total(&self, query: Option<String>) -> Result<Stat, DatabaseError> {
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
    async fn data_sources(&self) -> Result<Stat, DatabaseError> {
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
    async fn data_videos(&self) -> Result<Stat, DatabaseError> {
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

    #[instrument]
    async fn data_segments(&self) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/segments",
            self.client.workspace.slug
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "data_segments".into(),
            value,
        })
    }

    #[instrument]
    async fn processes_all(&self, process: &str) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/segments",
            self.client.workspace.slug
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "processes_all".into(),
            value,
        })
    }

    #[instrument]
    async fn investigations_total(&self) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/investigations",
            self.client.workspace.slug
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigations_total".into(),
            value,
        })
    }

    #[instrument]
    async fn investigation_segments_total(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/investigations/{}/segments",
            self.client.workspace.slug, investigation
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigation_segments_total".into(),
            value,
        })
    }
}
