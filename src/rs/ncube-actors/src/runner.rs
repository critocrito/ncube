use async_trait::async_trait;
use ncube_data::{Task, TaskKind, TaskState};
use ncube_tasks::{create_workspace, remove_location, run_data_process};
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;
use xactor::{message, Actor, Context, Handler};

use crate::{
    db::{DatabaseActor, MigrateWorkspace},
    host::{EnableWorkspace, HostActor},
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
                    TaskKind::SetupWorkspace {
                        location,
                        workspace,
                    } => {
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
                            .call(MigrateWorkspace {
                                workspace: workspace.to_string(),
                            })
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

                        let host_actor = HostActor::from_registry().await.unwrap();
                        host_actor
                            .call(EnableWorkspace { workspace })
                            .await
                            .unwrap()
                            .unwrap();
                    }

                    TaskKind::RemoveLocation { location } => {
                        info!("Received a request to remove a location: {:?}.", location,);

                        let actor = TaskActor::from_registry().await.unwrap();
                        actor
                            .call(UpdateTask {
                                task_id: task_id.clone(),
                                state: TaskState::Running,
                            })
                            .await
                            .unwrap()
                            .unwrap();

                        remove_location(location)
                            .await
                            .expect("Failed to remove a location");

                        actor
                            .call(UpdateTask {
                                task_id,
                                state: TaskState::Done,
                            })
                            .await
                            .unwrap()
                            .unwrap();
                    }

                    TaskKind::RunProcess {
                        workspace,
                        process_name,
                    } => {
                        info!(
                            "Received a request to run a process: {} -> {}.",
                            workspace.slug, process_name
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

                        run_data_process(workspace, &process_name)
                            .await
                            .expect("Failed to run process");

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
    pub task: Task,
}

#[async_trait]
impl Handler<QueueTask> for TaskRunner {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: QueueTask) -> Result<(), ActorError> {
        self.tx.send((msg.task.task_id(), msg.task.kind)).await?;
        Ok(())
    }
}
