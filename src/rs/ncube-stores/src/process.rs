use async_trait::async_trait;
use ncube_data::{Process, ProcessConfig, ProcessConfigKind, ProcessRunKind, ProcessRunReq};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_json::Value;
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
    async fn configure(
        &self,
        workspace: &str,
        capability: &str,
        value: &Value,
    ) -> Result<(), DatabaseError>;
    async fn run(&self, key: &str, kind: ProcessRunKind) -> Result<(), DatabaseError>;
    async fn is_configured(&self, workspace: &str, process: &str) -> Result<bool, DatabaseError>;
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

    async fn configure(
        &self,
        workspace: &str,
        capability: &str,
        value: &Value,
    ) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/process/show_capability.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/process/insert_config.sql"))?;

        let cap_iter = from_rows::<i32>(stmt.query(params![&workspace, &capability])?);

        for cap in cap_iter {
            let cap = cap?;
            stmt2.execute(params![cap, &value])?;
        }

        Ok(())
    }

    async fn run(&self, _key: &str, _kind: ProcessRunKind) -> Result<(), DatabaseError> {
        unreachable!()
    }

    #[instrument]
    async fn is_configured(&self, workspace: &str, process: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let result: i32 = conn.query_row(
            include_str!("../sql/process/is_configured.sql"),
            params![&workspace, &process],
            |row| row.get(0),
        )?;

        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
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
        unreachable!();
    }

    async fn configure(
        &self,
        _workspace: &str,
        _capability: &str,
        _value: &Value,
    ) -> Result<(), DatabaseError> {
        unreachable!()
    }

    async fn run(&self, key: &str, kind: ProcessRunKind) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/processes",
            self.client.workspace.slug
        ));

        let payload = ProcessRunReq {
            key: key.to_string(),
            kind,
        };

        self.client.post::<(), ProcessRunReq>(url, payload).await?;

        Ok(())
    }

    async fn is_configured(&self, _workspace: &str, _process: &str) -> Result<bool, DatabaseError> {
        unreachable!()
    }
}
