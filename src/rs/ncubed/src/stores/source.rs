use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{QueryTag, Source};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;
use tracing::instrument;

use crate::db::{http, sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn source_store(wrapped_db: Database) -> Box<dyn SourceStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(SourceStoreSqlite { db }),
        Database::Http(client) => Box::new(SourceStoreHttp { client }),
    }
}

#[async_trait]
pub(crate) trait SourceStore {
    async fn exists(&self, id: i32) -> Result<bool, StoreError>;
    async fn create(&self, kind: &str, term: &str, tags: Vec<QueryTag>) -> Result<(), StoreError>;
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Source>, StoreError>;
    async fn delete(&self, id: i32) -> Result<(), StoreError>;
    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError>;
    async fn list_source_tags(&self) -> Result<Vec<QueryTag>, StoreError>;
}

#[derive(Debug)]
pub struct SourceStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl SourceStore for SourceStoreSqlite {
    #[instrument]
    async fn exists(&self, id: i32) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&id], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
    async fn create(&self, kind: &str, term: &str, tags: Vec<QueryTag>) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/source/create.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/source/show.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/source/create-query-tag.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/source/show-query-tag.sql"))?;
        let mut stmt5 =
            conn.prepare_cached(include_str!("../sql/source/create-tagged-query.sql"))?;

        conn.execute_batch("BEGIN;")?;

        stmt.execute(params![&kind, &term, &now.to_rfc3339(), &now.to_rfc3339()])?;

        let query_id: i32 = stmt2.query_row(params![&kind, &term], |row| row.get(0))?;

        for tag in tags {
            stmt3.execute(params![&tag.label, &tag.description])?;
            let query_tag_id: i32 = stmt4.query_row(params![&tag.label], |row| row.get(0))?;
            stmt5.execute(params![query_id, query_tag_id])?;
        }

        conn.execute_batch("COMMIT")?;

        Ok(())
    }

    #[instrument]
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Source>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/paginate.sql"))?;
        let mut stmt2 =
            conn.prepare_cached(include_str!("../sql/source/list-query-tags-for-query.sql"))?;

        let offset = page * page_size;
        let mut sources: Vec<Source> = vec![];

        for source in from_rows::<Source>(stmt.query(params![offset, page_size])?) {
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

    #[instrument]
    async fn delete(&self, id: i32) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/source/delete-tagged-query.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/source/delete.sql"))?;

        stmt.execute(params![&id])?;
        stmt2.execute(params![&id])?;

        Ok(())
    }

    #[instrument]
    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/update.sql"))?;

        stmt.execute(params![&id, &kind, &term, &now.to_rfc3339()])?;

        Ok(())
    }

    #[instrument]
    async fn list_source_tags(&self) -> Result<Vec<QueryTag>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/list-source-tags.sql"))?;

        let mut source_tags: Vec<QueryTag> = vec![];

        for query_tag in from_rows::<QueryTag>(stmt.query(NO_PARAMS)?) {
            source_tags.push(query_tag?);
        }

        Ok(source_tags)
    }
}

#[derive(Debug)]
pub struct SourceStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl SourceStore for SourceStoreHttp {
    async fn exists(&self, _id: i32) -> Result<bool, StoreError> {
        todo!()
    }

    async fn create(
        &self,
        _kind: &str,
        _term: &str,
        _tags: Vec<QueryTag>,
    ) -> Result<(), StoreError> {
        todo!()
    }

    #[instrument]
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Source>, StoreError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/sources",
            self.client.workspace.slug
        ));
        url.query_pairs_mut()
            .clear()
            .append_pair("page", &page.to_string())
            .append_pair("size", &page_size.to_string());

        let data: Vec<Source> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    async fn delete(&self, _id: i32) -> Result<(), StoreError> {
        todo!()
    }

    async fn update(&self, _id: i32, _kind: &str, _term: &str) -> Result<(), StoreError> {
        todo!()
    }

    async fn list_source_tags(&self) -> Result<Vec<QueryTag>, StoreError> {
        todo!()
    }
}
