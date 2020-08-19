use async_trait::async_trait;
use ncube_data::Stat;
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use ncube_search::SearchQuery;
use rusqlite::{params, ToSql, NO_PARAMS};
use tracing::instrument;

use crate::SearchQuerySqlite;

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
    async fn data_total(&self, query: Option<SearchQuery>) -> Result<Stat, DatabaseError>;
    async fn data_sources(&self) -> Result<Stat, DatabaseError>;
    async fn data_videos(&self) -> Result<Stat, DatabaseError>;
    async fn data_segments(&self) -> Result<Stat, DatabaseError>;
    async fn processes_all(&self, process: &str) -> Result<Stat, DatabaseError>;
    async fn investigations_total(&self) -> Result<Stat, DatabaseError>;
    async fn investigation_data_total(&self, investigation: &str) -> Result<Stat, DatabaseError>;
    async fn investigation_data_verified(&self, investigation: &str)
        -> Result<Stat, DatabaseError>;
    async fn investigation_segments_total(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError>;
    async fn investigation_segment_verified(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError>;
    async fn investigation_segment_progress(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError>;
    async fn verified_total(&self) -> Result<Stat, DatabaseError>;
    async fn in_process_total(&self) -> Result<Stat, DatabaseError>;
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
    async fn data_total(&self, query: Option<SearchQuery>) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;

        let count_sources: i32 = match query {
            Some(query) => {
                let tmpl = include_str!("../sql/stat/count_units_search.sql");
                let params: Vec<Box<dyn ToSql>> = vec![];

                let sql = SearchQuerySqlite::from(&query);
                let (data_sql, params) = sql.to_sql(tmpl, params);
                let mut stmt = conn.prepare_cached(&data_sql)?;
                stmt.query_row(params, |row| row.get(0))?
            }
            None => {
                let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_units.sql"))?;
                stmt.query_row(NO_PARAMS, |row| row.get(0))?
            }
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
    async fn investigation_data_total(&self, investigation: &str) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/stat/count_investigation_data.sql"))?;

        let count: i32 = stmt.query_row(params![&investigation], |row| row.get(0))?;

        Ok(Stat {
            name: "investigation_data_total".into(),
            value: count,
        })
    }

    #[instrument]
    async fn investigation_data_verified(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/stat/count_investigation_verified.sql"))?;

        let count: i32 = stmt.query_row(params![&investigation], |row| row.get(0))?;

        Ok(Stat {
            name: "investigation_data_verified".into(),
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

    async fn investigation_segment_verified(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!(
            "../sql/stat/count_investigation_segment_verified.sql"
        ))?;

        let count: i32 = stmt.query_row(params![&investigation, &segment], |row| row.get(0))?;

        Ok(Stat {
            name: "investigation_segment_verified".into(),
            value: count,
        })
    }

    async fn investigation_segment_progress(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!(
            "../sql/stat/count_investigation_segment_progress.sql"
        ))?;

        let count: i32 = stmt.query_row(params![&investigation, &segment], |row| row.get(0))?;

        Ok(Stat {
            name: "investigation_segment_progress".into(),
            value: count,
        })
    }

    async fn verified_total(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_verified.sql"))?;

        let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "verified_total".into(),
            value: count,
        })
    }

    async fn in_process_total(&self) -> Result<Stat, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/stat/count_in_process.sql"))?;

        let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

        Ok(Stat {
            name: "in_process_total".into(),
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
    async fn data_total(&self, query: Option<SearchQuery>) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/data/total",
            self.client.workspace.slug
        ));

        if let Some(q) = query {
            url.query_pairs_mut()
                .clear()
                .append_pair("q", &q.to_string());
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
    async fn processes_all(&self, _process: &str) -> Result<Stat, DatabaseError> {
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
    async fn investigation_data_total(&self, investigation: &str) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/investigations/{}/data",
            self.client.workspace.slug, investigation
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigation_data_total".into(),
            value,
        })
    }

    #[instrument]
    async fn investigation_data_verified(
        &self,
        investigation: &str,
    ) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/investigations/{}/verified",
            self.client.workspace.slug, investigation
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigation_data_verified".into(),
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

    async fn investigation_segment_verified(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/investigations/{}/segments/{}/verified",
            self.client.workspace.slug, investigation, segment
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigation_segment_verified".into(),
            value,
        })
    }

    async fn investigation_segment_progress(
        &self,
        investigation: &str,
        segment: &str,
    ) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/investigations/{}/segments/{}/progress",
            self.client.workspace.slug, investigation, segment
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "investigation_segment_verified".into(),
            value,
        })
    }

    async fn verified_total(&self) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/verified",
            self.client.workspace.slug
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "verified_total".into(),
            value,
        })
    }

    async fn in_process_total(&self) -> Result<Stat, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/stats/in_process",
            self.client.workspace.slug
        ));

        let value: i32 = self.client.get(url).await?.unwrap_or_else(|| 0);

        Ok(Stat {
            name: "in_process_total".into(),
            value,
        })
    }
}
