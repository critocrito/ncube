use ncube_actors::{task::ListTasks, Registry, TaskActor};
use ncube_data::Task;
use tracing::instrument;

use crate::{ensure_workspace, HandlerError};

#[instrument]
pub async fn list(workspace: &str) -> Result<Vec<Task>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let actor = TaskActor::from_registry().await.unwrap();
    let tasks = actor.call(ListTasks).await??;

    Ok(tasks)
}
