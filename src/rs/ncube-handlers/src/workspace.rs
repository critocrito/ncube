use ncube_actors_common::Registry;
use ncube_actors_host::{HostActor, RequirePool, WorkspaceRootSetting};
use ncube_actors_task::{RemoveLocation, SetupWorkspace, TaskActor};

use ncube_data::{
    AccountRequest, DatabaseRequest, Investigation, InvestigationReq, Methodology, MethodologyReq,
    Segment, SegmentRequest, Unit, Workspace, WorkspaceDatabase, WorkspaceKind,
    WorkspaceKindRequest, WorkspaceRequest,
};
use ncube_db::{migrations, sqlite, DatabaseError};
use ncube_search::parse_query;
use ncube_stores::{
    investigation_store, methodology_store, search_store, segment_store, unit_store,
    workspace_store, WorkspaceStore,
};
use tokio::fs::File;
use tracing::{debug, instrument};

use crate::{account, ensure_workspace, lookup_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn create_workspace(workspace_req: WorkspaceRequest) -> Result<Workspace, HandlerError> {
    let workspace = workspace_req.slug();
    let WorkspaceRequest {
        name, description, ..
    } = workspace_req;
    let host_actor = HostActor::from_registry().await.unwrap();

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
            let actor = TaskActor::from_registry().await.unwrap();
            actor
                .call(SetupWorkspace {
                    location,
                    workspace: workspace.clone(),
                })
                .await??;
        }
        WorkspaceKindRequest::Remote {
            endpoint, account, ..
        } => {
            let kind = "remote".to_string();
            let location = endpoint.clone();
            let database = match workspace_req.database {
                DatabaseRequest::Http => "http",
                // Remote workspaces can only have HTTP databases.
                _ => {
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
pub async fn show_workspace(workspace: &str) -> Result<Workspace, HandlerError> {
    let workspace = lookup_workspace(workspace).await?;

    Ok(workspace)
}

#[instrument]
pub async fn list_workspaces() -> Result<Vec<Workspace>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let database = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(database.clone());

    let workspaces = workspace_store.list().await?;

    Ok(workspaces)
}

#[instrument]
pub async fn remove_workspace(workspace: &str, remove_location: bool) -> Result<(), HandlerError> {
    let workspace = lookup_workspace(workspace).await?;
    let host_actor = HostActor::from_registry().await.unwrap();
    let database = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(database.clone());

    // I can't chain if let with the second predicate, hence the nesting.
    // https://github.com/rust-lang/rust/issues/53667
    if let WorkspaceKind::Local(location) = &workspace.kind {
        if remove_location {
            let actor = TaskActor::from_registry().await.unwrap();
            actor
                .call(RemoveLocation {
                    workspace: workspace.clone(),
                    location: location.clone(),
                })
                .await??;
        }
    }

    workspace_store.delete_by_slug(&workspace.slug).await?;

    Ok(())
}

#[instrument]
pub async fn update_workspace(
    workspace_slug: &str,
    workspace_req: WorkspaceRequest,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace_slug).await?;

    let database = workspace_database(&workspace_slug).await?;
    let workspace_store = workspace_store(database.clone());

    let workspace = workspace_req.slug();
    let WorkspaceRequest {
        name, description, ..
    } = workspace_req;

    match workspace_req.kind {
        WorkspaceKindRequest::Local => {
            workspace_store
                .update(&workspace_slug.to_string(), &name, &workspace, &description)
                .await?
        }
        WorkspaceKindRequest::Remote { .. } => unimplemented!(),
    };

    Ok(())
}

#[instrument]
pub async fn list_data(
    workspace: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Unit>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let unit_store = unit_store(database);

    let data = unit_store.list(page, page_size).await?;

    Ok(data)
}

#[instrument]
pub async fn search_data(
    workspace: &str,
    query: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Unit>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let search_store = search_store(database);

    let search_query = parse_query(&query);
    let data = search_store.data(&search_query, page, page_size).await?;

    Ok(data)
}

#[instrument]
pub async fn migrate(workspace: &str) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();
    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db);
    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let connection_string = workspace.connection_string();

    match workspace.database {
        WorkspaceDatabase::Sqlite { .. } => {
            debug!("Matched SQLite database at {:?}", connection_string);

            let db = sqlite::Database::from_str(&connection_string, 1)
                .map_err(|e| HandlerError::Database(DatabaseError::SqliteConfig(e)))?;
            let mut conn = db
                .connection()
                .await
                .map_err(|e| HandlerError::Database(DatabaseError::SqlitePool(e)))?;
            migrations::migrate_workspace(&mut **conn)?;
            Ok(())
        }
        _ => Err(HandlerError::Invalid(format!(
            "The database as {:?} can't be migrated.",
            connection_string
        ))),
    }
}

#[instrument]
pub async fn create_segment(
    workspace: &str,
    segment_req: &SegmentRequest,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let segment_store = segment_store(database);

    if let Ok(true) = segment_store.exists(&segment_req.slug()).await {
        return Err(HandlerError::Invalid(format!(
            "Segment `{}` already exists.",
            segment_req.slug(),
        )));
    };

    segment_store
        .create(&segment_req.query, &segment_req.title, &segment_req.slug())
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_segment(workspace: &str, slug: &str) -> Result<Segment, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let segment_store = segment_store(database);
    let segment = segment_store.show(&slug).await?;

    segment.ok_or_else(|| HandlerError::NotFound(format!("Segment '{}' could not be found.", slug)))
}

#[instrument]
pub async fn list_segments(workspace: &str) -> Result<Vec<Segment>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let segment_store = segment_store(database);
    let segments = segment_store.list().await?;

    Ok(segments)
}

#[instrument]
pub async fn remove_segment(workspace: &str, slug: &str) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let segment_store = segment_store(database);
    segment_store.delete(&slug).await?;

    Ok(())
}

#[instrument]
pub async fn update_segment(
    workspace: &str,
    slug: &str,
    segment_req: &SegmentRequest,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let segment_store = segment_store(database);

    if let Ok(false) = segment_store.exists(&slug).await {
        return Err(HandlerError::NotFound(format!(
            "Segment '{}' could not be found.",
            slug
        )));
    }

    segment_store
        .update(&slug, &segment_req.query, &segment_req.title)
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_download(workspace: &str, file_path: &str) -> Result<File, HandlerError> {
    let workspace = lookup_workspace(workspace).await?;

    match workspace.kind {
        WorkspaceKind::Local(location) => {
            match File::open(format!("{}/data/{}", &location, &file_path)).await {
                Ok(file) => Ok(file),
                Err(err) => match err.kind() {
                    std::io::ErrorKind::NotFound => Err(HandlerError::NotFound(file_path.into())),
                    _ => Err(HandlerError::NotAllowed(file_path.into())),
                },
            }
        }
        WorkspaceKind::Remote(_) => {
            // FIXME: Downloads only work on local workspaces right now. To
            // support remote workspaces as well I will probably need to
            // refactor to return something else than `File`.
            todo!()
        }
    }
}

#[instrument]
pub async fn create_methodology(
    workspace: &str,
    methodology_req: &MethodologyReq,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let methodology_store = methodology_store(database);

    if let Ok(true) = methodology_store.exists(&methodology_req.slug()).await {
        return Err(HandlerError::Invalid(format!(
            "Methodology `{}` already exists.",
            methodology_req.slug(),
        )));
    };

    methodology_store
        .create(
            &methodology_req.title,
            &methodology_req.description,
            &methodology_req.process,
            &methodology_req.slug(),
        )
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_methodology(workspace: &str, slug: &str) -> Result<Methodology, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let methodology_store = methodology_store(database);
    let methodology = methodology_store.show(&slug).await?;

    methodology.ok_or_else(|| {
        HandlerError::NotFound(format!("Methodology '{}' could not be found.", slug))
    })
}

#[instrument]
pub async fn list_methodologies(workspace: &str) -> Result<Vec<Methodology>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let methodology_store = methodology_store(database);
    let methodologies = methodology_store.list().await?;

    Ok(methodologies)
}

#[instrument]
pub async fn create_investigation(
    workspace: &str,
    investigation_req: &InvestigationReq,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let methodology_store = methodology_store(database.clone());
    let investigation_store = investigation_store(database);

    let methodology = methodology_store
        .show(&investigation_req.methodology)
        .await?
        .ok_or_else(|| {
            HandlerError::NotFound(format!(
                "Methodology '{}' could not be found.",
                investigation_req.methodology
            ))
        })?;

    investigation_store
        .create(
            &investigation_req.title,
            &investigation_req.description,
            &methodology.slug,
            &investigation_req.slug(),
        )
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_investigation(
    workspace: &str,
    slug: &str,
) -> Result<Investigation, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let investigation_store = investigation_store(database);
    let investigation = investigation_store.show(&slug).await?;

    investigation.ok_or_else(|| {
        HandlerError::NotFound(format!("Investigation '{}' could not be found.", slug))
    })
}

#[instrument]
pub async fn list_investigations(workspace: &str) -> Result<Vec<Investigation>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let investigation_store = investigation_store(database);
    let investigations = investigation_store.list().await?;

    Ok(investigations)
}
