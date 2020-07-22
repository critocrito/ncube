use async_trait::async_trait;
use ncube_data::{Download, Media, QueryTag, SearchResponse, Source, Unit};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;
use tracing::instrument;

pub fn search_store(wrapped_db: Database) -> Box<dyn SearchStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(SearchStoreSqlite { db }),
        Database::Http(client) => Box::new(SearchStoreHttp { client }),
    }
}

#[async_trait]
pub trait SearchStore {
    async fn unit_index(&self) -> Result<(), DatabaseError>;
    async fn source_index(&self) -> Result<(), DatabaseError>;
    async fn data(
        &self,
        query: &str,
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
    async fn unit_index(&self) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/search/unit_index.sql"))?;
        stmt.execute(NO_PARAMS)?;

        Ok(())
    }

    async fn source_index(&self) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/search/source_index.sql"))?;
        stmt.execute(NO_PARAMS)?;

        Ok(())
    }

    #[instrument]
    async fn data(
        &self,
        query: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Unit>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/search/data.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/unit/list-media.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/unit/list-downloads.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/unit/list-sources.sql"))?;

        let offset = page * page_size;
        let mut units: Vec<Unit> = vec![];

        for unit in
            from_rows::<Unit>(stmt.query(params![&query, offset as i32, page_size as i32])?)
        {
            let mut unit = unit?;
            let mut medias: Vec<Media> = vec![];
            let mut downloads: Vec<Download> = vec![];
            let mut sources: Vec<Source> = vec![];

            for media in from_rows::<Media>(stmt2.query(params![unit.id])?) {
                medias.push(media?);
            }

            for download in from_rows::<Download>(stmt3.query(params![unit.id])?) {
                downloads.push(download?);
            }

            for source in from_rows::<Source>(stmt4.query(params![unit.id])?) {
                sources.push(source?);
            }

            unit.media = medias;
            unit.downloads = downloads;
            unit.sources = sources;

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
    async fn unit_index(&self) -> Result<(), DatabaseError> {
        unreachable!()
    }

    #[instrument]
    async fn source_index(&self) -> Result<(), DatabaseError> {
        unreachable!()
    }

    #[instrument]
    async fn data(
        &self,
        query: &str,
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
            .append_pair("q", &query)
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
