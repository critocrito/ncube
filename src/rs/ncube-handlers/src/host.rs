use ncube_actors::{
    host::{ClientSubscription, HostActor, RegisterClient, UnregisterClient, UpdateSubscription},
    task::{RunProcess, TaskActor},
};
use ncube_actors_common::Registry;
use ncube_data::{Client, Process, ProcessConfigReq, ProcessRunReq, WorkspaceKind};
use ncube_stores::process_store;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, instrument};

use crate::{database, lookup_workspace, HandlerError};

/// Our global unique user id counter.
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

#[instrument]
pub async fn list_processes(workspace: &str) -> Result<Vec<Process>, HandlerError> {
    let workspace = lookup_workspace(workspace).await?;
    let database = database(&workspace).await?;

    let process_store = process_store(database);
    let processes = process_store.list(&workspace.slug).await?;

    Ok(processes)
}

#[instrument]
pub async fn configure_process(
    workspace: &str,
    request: &ProcessConfigReq,
) -> Result<(), HandlerError> {
    let workspace = lookup_workspace(workspace).await?;
    let database = database(&workspace).await?;

    let process_store = process_store(database);
    process_store
        .configure(&workspace.slug, &request.key, &request.value)
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
    let workspace = lookup_workspace(workspace).await?;
    let database = database(&workspace).await?;

    let process_store = process_store(database);

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
                    process_name: request.key.clone(),
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

#[instrument]
pub async fn register_client() -> Result<String, HandlerError> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    let actor = HostActor::from_registry().await.unwrap();

    let uuid = actor.call(RegisterClient { client_id }).await??;

    debug!("Register client {} ({}).", client_id, uuid);

    Ok(uuid)
}

#[instrument]
pub async fn unregister_client(uuid: &str) -> Result<(), HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    debug!("Unregister client {}", uuid);

    actor
        .call(UnregisterClient {
            uuid: uuid.to_string(),
        })
        .await??;

    Ok(())
}

#[instrument]
pub async fn client_subscription(uuid: &str) -> Result<Option<Client>, HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    let client = actor
        .call(ClientSubscription {
            uuid: uuid.to_string(),
        })
        .await??;

    Ok(client)
}

#[instrument]
pub async fn update_subscription(uuid: &str, client: Client) -> Result<(), HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    actor
        .call(UpdateSubscription {
            uuid: uuid.to_string(),
            client,
        })
        .await??;

    Ok(())
}
