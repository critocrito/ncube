use async_trait::async_trait;
use ncube_data::{Process, ProcessConfig, ProcessConfigKind};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;
use tracing::instrument;

pub fn process_store(wrapped_db: Database) -> Box<dyn ProcessStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(ProcessStoreSqlite { db }),
        Database::Http(client) => Box::new(ProcessStoreHttp { client }),
    }
}

#[async_trait]
pub trait ProcessStore {
    async fn list(&self, workspace: &str) -> Result<Vec<Process>, DatabaseError>;
    async fn bootstrap(&self, workspace: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct ProcessStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl ProcessStore for ProcessStoreSqlite {
    async fn list(&self, workspace: &str) -> Result<Vec<Process>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/process/list.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/process/capabilities.sql"))?;

        let mut processes: Vec<Process> = vec![];
        for process in from_rows::<Process>(stmt.query(NO_PARAMS)?) {
            let mut process = process?;
            let mut configs: Vec<ProcessConfig> = vec![];

            let config_iter = stmt2.query_map(params![&workspace, process.id], |row| {
                Ok(ProcessConfig {
                    name: row.get(0)?,
                    key: row.get(1)?,
                    description: row.get(2)?,
                    kind: ProcessConfigKind::Secret,
                    template: serde_json::from_str(row.get::<usize, String>(4)?.as_str()).map_err(
                        |e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        },
                    )?,
                    value: match row.get::<usize, Option<String>>(5)? {
                        None => None,
                        Some(v) => serde_json::from_str(v.as_str()).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                5,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                    },
                })
            })?;

            for config in config_iter {
                configs.push(config?);
            }

            process.config = configs;
            processes.push(process)
        }

        Ok(processes)
    }

    async fn bootstrap(&self, workspace: &str) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/process/workspace_id.sql"))?;
        let mut stmt2 =
            conn.prepare_cached(include_str!("../sql/process/bootstrap_for_workspace.sql"))?;

        let workspace_id: i32 = stmt.query_row(params![&workspace], |row| row.get(0))?;

        stmt2.execute(params![workspace_id])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ProcessStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl ProcessStore for ProcessStoreHttp {
    #[instrument]
    async fn list(&self, _workspace: &str) -> Result<Vec<Process>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/processes",
            self.client.workspace.slug
        ));

        let data: Vec<Process> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    async fn bootstrap(&self, _workspace: &str) -> Result<(), DatabaseError> {
        unimplemented!();
    }
}
