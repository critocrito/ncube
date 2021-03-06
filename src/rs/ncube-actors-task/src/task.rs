use async_trait::async_trait;
use ncube_actors_common::{message, Actor, ActorError, Addr, Context, Handler, Registry};
use ncube_data::{ProcessRunKind, Task, TaskKind, TaskState, Workspace};
use ncube_tasks::TaskCache;
use std::fmt::Debug;
use tracing::info;

use crate::runner::{QueueTask, TaskRunner};

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
        self.cache.put(&task.task_id(), task.clone());
        self.runner.call(QueueTask { task }).await??;
        Ok(())
    }
}

#[message(result = "Result<Vec<Task>, ActorError>")]
#[derive(Debug)]
pub struct ListTasks;

#[async_trait]
impl Handler<ListTasks> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: ListTasks,
    ) -> Result<Vec<Task>, ActorError> {
        let tasks: Vec<Task> = self.cache.entries().into_iter().map(|(_, v)| v).collect();

        Ok(tasks)
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
    pub workspace: String,
    pub location: String,
}

#[async_trait]
impl Handler<SetupWorkspace> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetupWorkspace,
    ) -> Result<(), ActorError> {
        let slug = msg.workspace.clone();
        let task = Task::new(
            TaskKind::SetupWorkspace {
                workspace: msg.workspace,
                location: msg.location,
            },
            &slug,
        );
        self.queue_task(task).await
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct RemoveLocation {
    pub workspace: Workspace,
    pub location: String,
}

#[async_trait]
impl Handler<RemoveLocation> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RemoveLocation,
    ) -> Result<(), ActorError> {
        let slug = msg.workspace.slug.clone();
        let task = Task::new(
            TaskKind::RemoveLocation {
                workspace: msg.workspace,
                location: msg.location,
            },
            &slug,
        );
        self.queue_task(task).await
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct RunProcess {
    pub workspace: Workspace,
    pub process_name: String,
    pub kind: ProcessRunKind,
}

#[async_trait]
impl Handler<RunProcess> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RunProcess,
    ) -> Result<(), ActorError> {
        let slug = msg.workspace.slug.clone();
        let task = Task::new(
            TaskKind::RunProcess {
                workspace: msg.workspace,
                process_name: msg.process_name,
            },
            &slug,
        );
        self.queue_task(task).await
    }
}
