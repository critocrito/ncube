use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Methodology, MethodologyReq};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};
use tracing::instrument;

pub fn methodology_store(wrapped_db: Database) -> Box<dyn MethodologyStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(MethodologyStoreSqlite { db }),
        Database::Http(client) => Box::new(MethodologyStoreHttp { client }),
    }
}

#[async_trait]
pub trait MethodologyStore {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError>;
    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        process: &serde_json::Value,
        slug: &str,
    ) -> Result<(), DatabaseError>;
    async fn show(&self, slug: &str) -> Result<Option<Methodology>, DatabaseError>;
    async fn list(&self) -> Result<Vec<Methodology>, DatabaseError>;
}

#[derive(Debug)]
pub struct MethodologyStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl MethodologyStore for MethodologyStoreSqlite {
    #[instrument]
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/methodology/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        process: &serde_json::Value,
        slug: &str,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/methodology/create.sql"))?;

        stmt.execute(params![
            &title,
            &slug,
            &description,
            &process,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        Ok(())
    }

    #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Methodology>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/methodology/show.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&slug], |row| {
            from_row_with_columns::<Methodology>(row, &columns)
        })?;

        let mut methodologies: Vec<Methodology> = vec![];
        for row in rows {
            methodologies.push(row?)
        }

        match methodologies.first() {
            Some(methodology) => Ok(Some(methodology.to_owned())),
            _ => Ok(None),
        }
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Methodology>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/methodology/list.sql"))?;

        let mut methodologies: Vec<Methodology> = vec![];
        for row in from_rows::<Methodology>(stmt.query(NO_PARAMS)?) {
            methodologies.push(row?)
        }

        Ok(methodologies)
    }
}

#[derive(Debug)]
pub struct MethodologyStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl MethodologyStore for MethodologyStoreHttp {
    #[instrument]
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/methodologies/{}",
            self.client.workspace.slug, slug,
        ));

        match self.client.get::<Methodology>(url).await {
            Ok(_) => Ok(true),
            _ => Ok(false),
        }
    }

    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        process: &serde_json::Value,
        _slug: &str,
    ) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/methodologies",
            self.client.workspace.slug
        ));

        let payload = MethodologyReq {
            title: title.to_string(),
            process: process.clone(),
            description: description.clone(),
        };

        self.client.post::<(), MethodologyReq>(url, payload).await?;

        Ok(())
    }

    #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Methodology>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/methodologies/{}",
            self.client.workspace.slug, slug,
        ));

        let data: Option<Methodology> = self.client.get(url).await?;

        Ok(data)
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Methodology>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/methodologies",
            self.client.workspace.slug
        ));

        let data: Vec<Methodology> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }
}
