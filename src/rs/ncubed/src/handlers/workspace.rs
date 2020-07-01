use ncube_data::{Stat, Unit, Workspace};
use tracing::{error, instrument};

use crate::actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{RequirePool, WorkspaceRootSetting},
    task::SetupWorkspace,
    HostActor, Registry, TaskActor,
};
use crate::errors::HandlerError;
use crate::handlers::account;
use crate::stores::{stat_store, unit_store, workspace_store, WorkspaceStore};
use crate::types::{AccountRequest, DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest};

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
        return Err(HandlerError::Invalid(format!(
            "Workspace `{}` already exists.",
            workspace
        )));
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
        WorkspaceKindRequest::Remote {
            endpoint, account, ..
        } => {
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

            let AccountRequest {
                email,
                password,
                password_again,
                otp,
            } = account;

            account::create_account(&workspace, &email, Some(otp.clone())).await?;
            account::update_password(&workspace, &email, &password, &password_again).await?;
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

#[instrument]
pub async fn stat_source(workspace: &str) -> Result<Vec<Stat>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.sources().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data(workspace: &str) -> Result<Vec<Stat>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.data().await?;

    Ok(stats)
}

#[instrument]
pub async fn list_data(
    workspace: &str,
    page: usize,
    page_size: usize,
) -> Result<Vec<Unit>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let unit_store = unit_store(database);

    let data = unit_store.list(page, page_size).await?;

    Ok(data)
}
