use async_trait::async_trait;
use ncube_data::{Download, Media, QueryTag, SearchResponse, Source, Unit};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use ncube_search::SearchQuery;
use rusqlite::{params, ToSql};
use serde_rusqlite::from_rows;
use tracing::instrument;

use crate::SearchQuerySqlite;

pub fn search_store(wrapped_db: Database) -> Box<dyn SearchStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(SearchStoreSqlite { db }),
        Database::Http(client) => Box::new(SearchStoreHttp { client }),
    }
}

#[async_trait]
pub trait SearchStore {
    async fn data(
        &self,
        query: &SearchQuery,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Unit>, DatabaseError>;
    async fn sources(
        &self,
        query: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Source>, DatabaseError>;
}

#[derive(Debug)]
pub struct SearchStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl SearchStore for SearchStoreSqlite {
    #[instrument]
    async fn data(
        &self,
        query: &SearchQuery,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Unit>, DatabaseError> {
        let conn = self.db.connection().await?;

        let tmpl = include_str!("../sql/search/data.sql");
        let offset = page * page_size;
        let params: Vec<Box<dyn ToSql>> = vec![Box::new(offset), Box::new(page_size)];

        let sql = SearchQuerySqlite::from(query);
        let (data_sql, params) = sql.to_sql(tmpl, params);

        let mut stmt = conn.prepare_cached(&data_sql)?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/unit/list-media.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/unit/list-downloads.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/unit/list-sources.sql"))?;
        let mut stmt5 = conn.prepare_cached(include_str!("../sql/unit/list-tags.sql"))?;

        let mut units: Vec<Unit> = vec![];

        for unit in from_rows::<Unit>(stmt.query(params)?) {
            let mut unit = unit?;
            let mut medias: Vec<Media> = vec![];
            let mut downloads: Vec<Download> = vec![];
            let mut sources: Vec<Source> = vec![];
            let mut tags: Vec<QueryTag> = vec![];

            for media in from_rows::<Media>(stmt2.query(params![unit.id])?) {
                medias.push(media?);
            }

            for download in from_rows::<Download>(stmt3.query(params![unit.id])?) {
                downloads.push(download?);
            }

            for source in from_rows::<Source>(stmt4.query(params![unit.id])?) {
                sources.push(source?);
            }

            for tag in from_rows::<QueryTag>(stmt5.query(params![unit.id])?) {
                tags.push(tag?);
            }

            unit.media = medias;
            unit.downloads = downloads;
            unit.sources = sources;
            unit.tags = tags;

            units.push(unit);
        }

        Ok(units)
    }

    #[instrument]
    async fn sources(
        &self,
        query: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Source>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/search/source.sql"))?;
        let mut stmt2 =
            conn.prepare_cached(include_str!("../sql/source/list-query-tags-for-query.sql"))?;

        let offset = page * page_size;
        let mut sources: Vec<Source> = vec![];

        for source in from_rows::<Source>(stmt.query(params![&query, offset, page_size])?) {
            let mut source = source?;

            let mut tags: Vec<QueryTag> = vec![];

            for tag in from_rows::<QueryTag>(stmt2.query(params![source.id])?) {
                tags.push(tag?)
            }

            source.tags = tags;

            sources.push(source);
        }

        Ok(sources)
    }
}

#[derive(Debug)]
pub struct SearchStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl SearchStore for SearchStoreHttp {
    #[instrument]
    async fn data(
        &self,
        query: &SearchQuery,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Unit>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/data/search",
            self.client.workspace.slug
        ));
        url.query_pairs_mut()
            .clear()
            .append_pair("q", &query.to_string())
            .append_pair("page", &page.to_string())
            .append_pair("size", &page_size.to_string());

        let data: SearchResponse<Unit> =
            self.client
                .get(url)
                .await?
                .unwrap_or_else(|| SearchResponse {
                    data: vec![],
                    total: 0,
                });

        Ok(data.data)
    }

    #[instrument]
    async fn sources(
        &self,
        query: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Source>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/sources/search",
            self.client.workspace.slug
        ));
        url.query_pairs_mut()
            .clear()
            .append_pair("q", &query)
            .append_pair("page", &page.to_string())
            .append_pair("size", &page_size.to_string());

        let data: SearchResponse<Source> =
            self.client
                .get(url)
                .await?
                .unwrap_or_else(|| SearchResponse {
                    data: vec![],
                    total: 0,
                });

        Ok(data.data)
    }
}
