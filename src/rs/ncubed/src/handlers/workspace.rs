use ncube_data::Workspace;
use tracing::{error, instrument};

use crate::actors::{
    host::{
        CreateWorkspace, ListWorkspaces, RemoveWorkspace, ShowWorkspace, UpdateWorkspace,
        WorkspaceExists,
    },
    HostActor,
};
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::types::WorkspaceRequest;

#[instrument]
pub async fn create_workspace(workspace: WorkspaceRequest) -> Result<(), HandlerError> {
    let slug = workspace.slug();
    let mut actor = HostActor::from_registry().await.unwrap();

    if let Ok(true) = actor.call(WorkspaceExists { slug }).await? {
        let msg = format!("Workspace `{}` already exists.", workspace.slug());
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    actor.call(CreateWorkspace::from(workspace)).await??;

    Ok(())
}

#[instrument]
pub async fn show_workspace(slug: &str) -> Result<Workspace, HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    let workspace = actor.call(ShowWorkspace { slug: slug.into() }).await??;

    Ok(workspace)
}

#[instrument]
pub async fn list_workspaces() -> Result<Vec<Workspace>, HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    let workspaces = actor.call(ListWorkspaces).await??;

    Ok(workspaces)
}

#[instrument]
pub async fn remove_workspace(slug: &str) -> Result<(), HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    actor.call(RemoveWorkspace { slug: slug.into() }).await??;

    Ok(())
}

#[instrument]
pub async fn update_workspace(
    current_slug: &str,
    workspace_request: WorkspaceRequest,
) -> Result<(), HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    if let Ok(false) = actor
        .call(WorkspaceExists {
            slug: current_slug.into(),
        })
        .await?
    {
        return Err(HandlerError::NotFound(format!(
            "Workspace/{}",
            current_slug
        )));
    };

    let slug = workspace_request.slug();

    let WorkspaceRequest {
        name, description, ..
    } = workspace_request;

    actor
        .call(UpdateWorkspace {
            current_slug: current_slug.to_string(),
            slug,
            name,
            description,
        })
        .await??;

    Ok(())
}
