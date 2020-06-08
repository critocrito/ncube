use ncube_data::Workspace;
use tracing::{error, instrument};

use crate::actors::{
    host::{RequirePool, WorkspaceRootSetting},
    task::SetupWorkspace,
    HostActor, TaskActor,
};
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{workspace_store, WorkspaceStore};
use crate::types::{DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest};

#[instrument]
pub async fn create_workspace(workspace: WorkspaceRequest) -> Result<Workspace, HandlerError> {
    let slug = workspace.slug();
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_root = host_actor.call(WorkspaceRootSetting).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(true) = workspace_store.exists(&slug).await {
        let msg = format!("Workspace `{}` already exists.", workspace.slug());
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let kind = match &workspace.kind {
        WorkspaceKindRequest::Local => "local".to_string(),
        WorkspaceKindRequest::Remote { .. } => "remote".to_string(),
    };
    let location = match &workspace.kind {
        WorkspaceKindRequest::Local => workspace_root
            .clone()
            .join(&workspace.slug())
            .to_string_lossy()
            .into_owned(),
        WorkspaceKindRequest::Remote { endpoint } => endpoint.clone(),
    };

    // Esnure that remote workspaces have a http database configured,
    let database = match workspace.database {
        DatabaseRequest::Sqlite => match &workspace.kind {
            WorkspaceKindRequest::Local => "sqlite",
            _ => Err(HandlerError::Invalid(
                "local workspaces don't work with a `http` database".into(),
            ))?,
        },
        DatabaseRequest::Http { .. } => match &workspace.kind {
            WorkspaceKindRequest::Remote { .. } => "http",
            _ => Err(HandlerError::Invalid(
                "remote workspaces require a `http` database".into(),
            ))?,
        },
    };

    let database_path = match workspace.database {
        DatabaseRequest::Sqlite => workspace_root
            .join(&workspace.slug())
            .join("sugarcube.db")
            .to_string_lossy()
            .into_owned(),
        DatabaseRequest::Http => location.clone(),
    };

    workspace_store
        .create(
            &workspace.name,
            &slug,
            &workspace.description,
            &kind,
            &location,
            &database,
            &database_path,
        )
        .await?;

    if let WorkspaceKindRequest::Local = workspace.kind {
        let mut actor = TaskActor::from_registry().await.unwrap();
        actor.call(SetupWorkspace { location }).await??;
    }

    let workspace = workspace_store.show_by_slug(&slug).await?;

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
    current_slug: &str,
    workspace_request: WorkspaceRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&current_slug).await {
        let msg = format!("Workspace `{}` already exists.", current_slug);
        error!("{:?}", msg);
        return Err(HandlerError::NotFound(format!(
            "Workspace/{}",
            current_slug
        )));
    };

    let slug = workspace_request.slug();

    let WorkspaceRequest {
        name, description, ..
    } = workspace_request;

    workspace_store
        .update(&current_slug.to_string(), &name, &slug, &description)
        .await?;

    Ok(())
}
