use ncube_actors_common::Registry;
use ncube_actors_task::{ListTasks, TaskActor};
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
