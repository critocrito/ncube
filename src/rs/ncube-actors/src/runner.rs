use async_trait::async_trait;
use ncube_tasks::{create_workspace, Task, TaskKind, TaskState};
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;
use xactor::{message, Actor, Context, Handler};

use crate::{
    db::{DatabaseActor, MigrateWorkspace},
    task::{TaskActor, UpdateTask},
    ActorError, Registry,
};

pub struct TaskRunner {
    tx: Sender<(String, TaskKind)>,
}

impl Actor for TaskRunner {}

impl TaskRunner {
    pub fn new() -> Self {
        let (tx, mut rx): (Sender<(String, TaskKind)>, Receiver<(String, TaskKind)>) =
            mpsc::channel(100);

        tokio::spawn(async move {
            while let Some((task_id, task)) = rx.recv().await {
                info!("Received a new task {:?} with id {}.", task, task_id);
                match task {
                    TaskKind::SetupWorkspace(location, workspace) => {
                        info!(
                            "Received a request to setup a workspace: {:?}/{:?}.",
                            location, workspace
                        );

                        let actor = TaskActor::from_registry().await.unwrap();
                        actor
                            .call(UpdateTask {
                                task_id: task_id.clone(),
                                state: TaskState::Running,
                            })
                            .await
                            .unwrap()
                            .unwrap();

                        create_workspace(location)
                            .await
                            .expect("Failed to create workspace");

                        let database_actor = DatabaseActor::from_registry().await.unwrap();
                        database_actor
                            .call(MigrateWorkspace { workspace })
                            .await
                            .unwrap()
                            .unwrap();

                        actor
                            .call(UpdateTask {
                                task_id,
                                state: TaskState::Done,
                            })
                            .await
                            .unwrap()
                            .unwrap();
                    }
                }
            }
        });

        TaskRunner { tx }
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct QueueTask {
    pub task_id: String,
    pub task: Task,
}

#[async_trait]
impl Handler<QueueTask> for TaskRunner {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: QueueTask) -> Result<(), ActorError> {
        self.tx.send((msg.task_id, msg.task.kind)).await?;
        Ok(())
    }
}
