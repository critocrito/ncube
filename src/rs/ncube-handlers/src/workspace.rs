use bytes::{Bytes, BytesMut};
use futures_util::{stream::Stream, TryFutureExt, TryStreamExt};
use ncube_actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{RequirePool, WorkspaceRootSetting},
    task::SetupWorkspace,
    HostActor, Registry, TaskActor,
};
use ncube_data::{
    AccountRequest, DatabaseRequest, Segment, SegmentRequest, Stat, Unit, Workspace,
    WorkspaceDatabase, WorkspaceKind, WorkspaceKindRequest, WorkspaceRequest,
};
use ncube_db::{migrations, sqlite, DatabaseError};
use ncube_stores::{
    search_store, segment_store, stat_store, unit_store, workspace_store, WorkspaceStore,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::{debug, error, instrument};

use crate::{account, ensure_workspace, workspace_database, HandlerError};

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
pub async fn show_workspace(slug: &str) -> Result<Workspace, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    let workspace = workspace_store.show_by_slug(&slug).await?;

    Ok(workspace)
}

#[instrument]
pub async fn list_workspaces() -> Result<Vec<Workspace>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    let workspaces = workspace_store.list().await?;

    Ok(workspaces)
}

#[instrument]
pub async fn remove_workspace(slug: &str) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

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
    let host_actor = HostActor::from_registry().await.unwrap();

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

pub async fn stat_sources_total(
    workspace: &str,
    query: Option<String>,
) -> Result<Stat, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.sources_total(query).await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_sources_types(workspace: &str) -> Result<Stat, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.sources_types().await?;

    Ok(stats)
}

pub async fn stat_data_total(workspace: &str, query: Option<String>) -> Result<Stat, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.data_total(query).await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_sources(workspace: &str) -> Result<Stat, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.data_sources().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_videos(workspace: &str) -> Result<Stat, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let stat_store = stat_store(database);

    let stats = stat_store.data_videos().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_segments(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.data_segments().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_process_all(workspace: &str, process: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.processes_all(&process).await?;

    Ok(stats)
}

#[instrument]
pub async fn list_data(
    workspace: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Unit>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

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
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let search_store = search_store(database);

    let data = search_store.data(&query, page, page_size).await?;

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
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

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
pub async fn show_download(
    workspace: &str,
    file_path: &str,
) -> Result<impl Stream<Item = Result<Bytes, std::io::Error>>, HandlerError> {
    let workspace = show_workspace(&workspace).await?;

    // When we list processes for a local workspace we need to use the local
    // sqlite host database, but for remote workspaces we still have to query
    // the remote workspace database.
    match workspace.kind {
        WorkspaceKind::Local(location) => {
            let s = File::open(format!("{}/data/{}", &location, &file_path))
                .map_ok(|file| FramedRead::new(file, BytesCodec::new()).map_ok(BytesMut::freeze))
                .try_flatten_stream();
            Ok(s)
        }
        WorkspaceKind::Remote(_) => {
            // let database_actor = DatabaseActor::from_registry().await.unwrap();
            // let db = database_actor
            //     .call(LookupDatabase {
            //         workspace: workspace.slug.to_string(),
            //     })
            //     .await??
            todo!()
        }
    }
}
