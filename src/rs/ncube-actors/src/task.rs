use async_trait::async_trait;
use ncube_data::{ProcessRunKind, Workspace};
use ncube_tasks::{Task, TaskCache, TaskState};
use std::fmt::Debug;
use tracing::info;
use uuid::Uuid;
use xactor::{message, Actor, Addr, Context, Handler};

use crate::{
    runner::{QueueTask, TaskRunner},
    ActorError, Registry,
};

pub struct TaskActor {
    cache: TaskCache,
    runner: Addr<TaskRunner>,
}

impl Actor for TaskActor {}

impl Registry for TaskActor {}

impl TaskActor {
    pub async fn new() -> Self {
        let runner = TaskRunner::new().start().await.unwrap();
        TaskActor {
            runner,
            cache: TaskCache::new(),
        }
    }

    async fn queue_task(&mut self, task: Task) -> Result<(), ActorError> {
        let mut encoding_buffer = Uuid::encode_buffer();
        let task_id = Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut encoding_buffer);

        self.cache.put(&task_id, task.clone());
        self.runner
            .call(QueueTask {
                task_id: task_id.to_string(),
                task,
            })
            .await??;
        Ok(())
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct UpdateTask {
    pub task_id: String,
    pub state: TaskState,
}

#[async_trait]
impl Handler<UpdateTask> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UpdateTask,
    ) -> Result<(), ActorError> {
        info!("Receiving a task update for {}", msg.task_id);
        let mut task = self.cache.get(&msg.task_id).ok_or_else(|| {
            ActorError::Invalid(format!("task with id {} not found", msg.task_id))
        })?;

        task.state = msg.state;
        self.cache.reset(&msg.task_id, task);

        Ok(())
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct SetupWorkspace {
    pub location: String,
    pub workspace: String,
}

#[async_trait]
impl Handler<SetupWorkspace> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetupWorkspace,
    ) -> Result<(), ActorError> {
        let task = Task::workspace(&msg.location, &msg.workspace);
        self.queue_task(task).await
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct RunProcess {
    pub workspace: Workspace,
    pub key: String,
    pub kind: ProcessRunKind,
}

#[async_trait]
impl Handler<RunProcess> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RunProcess,
    ) -> Result<(), ActorError> {
        let task = Task::data_process(&msg.workspace, &msg.key);
        self.queue_task(task).await
    }
}
