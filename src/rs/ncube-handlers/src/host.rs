use ncube_actors::{host::RequirePool, HostActor, Registry};
use ncube_data::Process;
use ncube_stores::process_store;
use tracing::instrument;

use crate::HandlerError;

#[instrument]
pub async fn list_processes(workspace: &str) -> Result<Vec<Process>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let process_store = process_store(db.clone());

    let processes = process_store.list(&workspace).await?;

    Ok(processes)
}
