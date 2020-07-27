use ncube_actors::{
    db::{DatabaseActor, LookupDatabase},
    host::RequirePool,
    HostActor, Registry,
};
use ncube_data::{Process, WorkspaceKind};
use ncube_stores::process_store;
use tracing::instrument;

use crate::{workspace::show_workspace, HandlerError};

#[instrument]
pub async fn list_processes(workspace_slug: &str) -> Result<Vec<Process>, HandlerError> {
    let workspace = show_workspace(&workspace_slug).await?;

    // When we list processes for a local workspace we need to use the local
    // sqlite host database, but for remote workspaces we still have to query
    // the remote workspace database.
    let db = match workspace.kind {
        WorkspaceKind::Local(_) => {
            let mut host_actor = HostActor::from_registry().await.unwrap();
            host_actor.call(RequirePool).await??
        }
        WorkspaceKind::Remote(_) => {
            let mut database_actor = DatabaseActor::from_registry().await.unwrap();
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
