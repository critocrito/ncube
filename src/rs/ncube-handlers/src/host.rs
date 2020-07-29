use ncube_actors::{
    db::{DatabaseActor, LookupDatabase},
    host::RequirePool,
    task::{RunProcess, TaskActor},
    HostActor, Registry,
};
use ncube_data::{Process, ProcessConfigReq, ProcessRunReq, WorkspaceKind};
use ncube_stores::process_store;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::instrument;

use crate::{workspace::show_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn list_processes(workspace_slug: &str) -> Result<Vec<Process>, HandlerError> {
    let workspace = show_workspace(&workspace_slug).await?;

    // When we list processes for a local workspace we need to use the local
    // sqlite host database, but for remote workspaces we still have to query
    // the remote workspace database.
    let db = match workspace.kind {
        WorkspaceKind::Local(_) => {
            let host_actor = HostActor::from_registry().await.unwrap();
            host_actor.call(RequirePool).await??
        }
        WorkspaceKind::Remote(_) => {
            let database_actor = DatabaseActor::from_registry().await.unwrap();
            database_actor
                .call(LookupDatabase {
                    workspace: workspace.slug.to_string(),
                })
                .await??
        }
    };

    let process_store = process_store(db);
    let processes = process_store.list(&workspace_slug).await?;

    Ok(processes)
}

#[instrument]
pub async fn configure_process(
    workspace_slug: &str,
    request: &ProcessConfigReq,
) -> Result<(), HandlerError> {
    let workspace = show_workspace(&workspace_slug).await?;

    // When we list processes for a local workspace we need to use the local
    // sqlite host database, but for remote workspaces we still have to query
    // the remote workspace database.
    let db = match workspace.kind {
        WorkspaceKind::Local(_) => {
            let host_actor = HostActor::from_registry().await.unwrap();
            host_actor.call(RequirePool).await??
        }
        WorkspaceKind::Remote(_) => {
            let database_actor = DatabaseActor::from_registry().await.unwrap();
            database_actor
                .call(LookupDatabase {
                    workspace: workspace.slug.to_string(),
                })
                .await??
        }
    };

    let process_store = process_store(db);
    process_store
        .configure(&workspace_slug, &request.key, &request.value)
        .await?;

    // We update the secrets.json file in the workspace as well.
    if let WorkspaceKind::Local(location) = workspace.kind {
        let mut secrets_path = Path::new(&location).to_path_buf();
        secrets_path.push("configs/secrets.json");

        let mut secrets: Value = if secrets_path.exists() {
            let secrets_file = File::open(secrets_path.as_path())
                .map_err(|e| HandlerError::Invalid(e.to_string()))?;
            let reader = BufReader::new(secrets_file);
            serde_json::from_reader(reader)
                .or_else(|_| serde_json::from_str::<Value>("{}"))
                .map_err(|e| HandlerError::Invalid(e.to_string()))?
        } else {
            serde_json::from_str::<Value>("{}").map_err(|e| HandlerError::Invalid(e.to_string()))?
        };

        secrets[&request.key] = request.value.clone();

        let file = File::create(secrets_path.as_path())
            .map_err(|e| HandlerError::Invalid(e.to_string()))?;
        serde_json::to_writer_pretty(file, &secrets)
            .map_err(|e| HandlerError::Invalid(e.to_string()))?;
    }

    Ok(())
}

#[instrument]
pub async fn run_process(workspace: &str, request: &ProcessRunReq) -> Result<(), HandlerError> {
    let workspace = show_workspace(&workspace).await?;

    let db = match workspace.kind {
        WorkspaceKind::Local(_) => {
            let host_actor = HostActor::from_registry().await.unwrap();
            host_actor.call(RequirePool).await??
        }
        WorkspaceKind::Remote(_) => workspace_database(&workspace.slug).await?,
    };
    let process_store = process_store(db);

    match workspace.kind {
        WorkspaceKind::Local(_) => {
            if let false = process_store
                .is_configured(&workspace.slug, &request.key)
                .await?
            {
                return Err(HandlerError::Invalid(format!(
                    "Process {} requires configuration",
                    &request.key
                )));
            }

            let actor = TaskActor::from_registry().await.unwrap();
            actor
                .call(RunProcess {
                    workspace,
                    key: request.key.clone(),
                    kind: request.kind.clone(),
                })
                .await??;
        }
        WorkspaceKind::Remote(_) => {
            process_store
                .run(&request.key, request.kind.clone())
                .await?;
        }
    };

    Ok(())
}
