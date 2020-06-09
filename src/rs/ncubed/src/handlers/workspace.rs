use ncube_data::Workspace;
use tracing::{error, instrument};

use crate::actors::{
    db::LookupDatabase,
    host::{RequirePool, WorkspaceRootSetting},
    task::SetupWorkspace,
    DatabaseActor, HostActor, Registry, TaskActor,
};
use crate::errors::HandlerError;
use crate::stores::{workspace_store, WorkspaceStore};
use crate::types::{DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest};

#[instrument]
pub async fn create_workspace(workspace_req: WorkspaceRequest) -> Result<Workspace, HandlerError> {
    let workspace = workspace_req.slug();
    let WorkspaceRequest {
        name, description, ..
    } = workspace_req;
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_root = host_actor.call(WorkspaceRootSetting).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(true) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` already exists.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    // Depending if we have a local or remote workspace we have to do different
    // things.
    match &workspace_req.kind {
        WorkspaceKindRequest::Local => {
            let kind = "local".to_string();
            let location = workspace_root
                .clone()
                .join(&workspace)
                .to_string_lossy()
                .into_owned();
            let database = match workspace_req.database {
                DatabaseRequest::Sqlite => "sqlite",
                // One cannot create a local workspace with a HTTP database backend.
                DatabaseRequest::Http => {
                    return Err(HandlerError::Invalid(
                        "local workspaces don't work with a `http` database".into(),
                    ))
                }
            };
            let database_path = match workspace_req.database {
                DatabaseRequest::Sqlite => workspace_root
                    .join(&workspace)
                    .join("sugarcube.db")
                    .to_string_lossy()
                    .into_owned(),
                DatabaseRequest::Http => unreachable!(),
            };

            workspace_store
                .create(
                    &name,
                    &workspace,
                    &description,
                    &kind,
                    &location,
                    &database,
                    &database_path,
                )
                .await?;
            let mut actor = TaskActor::from_registry().await.unwrap();
            actor.call(SetupWorkspace { location }).await??;
        }
        WorkspaceKindRequest::Remote { endpoint, .. } => {
            let kind = "remote".to_string();
            let location = endpoint.clone();
            let database = match workspace_req.database {
                DatabaseRequest::Http => "http",
                // Remote workspaces can only have HTTP databases.
                DatabaseRequest::Sqlite => {
                    return Err(HandlerError::Invalid(
                        "remote workspaces require a `http` database".into(),
                    ))
                }
            };

            workspace_store
                .create(
                    &workspace, &workspace, &None, &kind, &location, &database, &endpoint,
                )
                .await?;
        }
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    Ok(workspace)
}

#[instrument]
pub async fn show_workspace(slug: &str) -> Result<Workspace, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    let workspace = workspace_store.show_by_slug(&slug).await?;

    Ok(workspace)
}

#[instrument]
pub async fn list_workspaces() -> Result<Vec<Workspace>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    let workspaces = workspace_store.list().await?;

    Ok(workspaces)
}

#[instrument]
pub async fn remove_workspace(slug: &str) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    workspace_store.delete_by_slug(&slug).await?;

    Ok(())
}

#[instrument]
pub async fn update_workspace(
    cur_workspace: &str,
    workspace_req: WorkspaceRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&cur_workspace).await {
        let msg = format!("Workspace `{}` already exists.", cur_workspace);
        error!("{:?}", msg);
        return Err(HandlerError::NotFound(format!(
            "Workspace/{}",
            cur_workspace
        )));
    };

    let workspace = workspace_req.slug();
    let WorkspaceRequest {
        name, description, ..
    } = workspace_req;

    match workspace_req.kind {
        WorkspaceKindRequest::Local => {
            workspace_store
                .update(&cur_workspace.to_string(), &name, &workspace, &description)
                .await?
        }
        WorkspaceKindRequest::Remote { .. } => unimplemented!(),
    };

    Ok(())
}
