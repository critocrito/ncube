use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Annotation, AnnotationReq};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::params;
use serde_rusqlite::{self, from_row};
use tracing::instrument;

pub fn annotation_store(wrapped_db: Database) -> Box<dyn AnnotationStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(AnnotationStoreSqlite { db }),
        Database::Http(client) => Box::new(AnnotationStoreHttp { client }),
    }
}

#[async_trait]
pub trait AnnotationStore {
    async fn create(
        &self,
        key: &str,
        value: &serde_json::Value,
        name: &str,
        note: &Option<String>,
        investigation: &str,
        verification: i32,
    ) -> Result<(), DatabaseError>;
    async fn list(
        &self,
        investigation: &str,
        verification: i32,
    ) -> Result<Vec<Annotation>, DatabaseError>;
}

#[derive(Debug)]
pub struct AnnotationStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl AnnotationStore for AnnotationStoreSqlite {
    #[instrument]
    async fn create(
        &self,
        key: &str,
        value: &serde_json::Value,
        name: &str,
        note: &Option<String>,
        _investigation: &str,
        verification: i32,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/annotation/create.sql"))?;

        stmt.execute(params![
            verification,
            &key,
            &value,
            &name,
            &note,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        Ok(())
    }

    async fn list(
        &self,
        _investigation: &str,
        verification: i32,
    ) -> Result<Vec<Annotation>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/annotation/list.sql"))?;

        let mut annotations: Vec<Annotation> = vec![];
        for row in stmt.query_and_then(params![verification], from_row::<Annotation>)? {
            annotations.push(row?)
        }

        Ok(annotations)
    }
}

#[derive(Debug)]
pub struct AnnotationStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl AnnotationStore for AnnotationStoreHttp {
    async fn create(
        &self,
        key: &str,
        value: &serde_json::Value,
        name: &str,
        note: &Option<String>,
        investigation: &str,
        verification: i32,
    ) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations/{}/annotations/{}",
            self.client.workspace.slug, investigation, verification
        ));

        let payload = AnnotationReq {
            key: key.to_string(),
            value: value.to_owned(),
            note: note.to_owned(),
            name: name.to_string(),
        };

        self.client.post::<(), AnnotationReq>(url, payload).await?;

        Ok(())
    }

    async fn list(
        &self,
        investigation: &str,
        verification: i32,
    ) -> Result<Vec<Annotation>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations/{}/annotations/{}",
            self.client.workspace.slug, investigation, verification
        ));

        let annotations: Vec<Annotation> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(annotations)
    }
}
