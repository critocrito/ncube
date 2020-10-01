use async_trait::async_trait;
use ncube_actors_common::{message, Actor, ActorError, Context, Handler, Registry};
use ncube_actors_db::{DatabaseActor, MigrateWorkspace};
use ncube_actors_host::{EnableWorkspace, HostActor};
use ncube_data::{Task, TaskKind};
use ncube_tasks::{create_workspace, remove_location, run_data_process};
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;

use crate::TaskLifecycle;

pub struct TaskRunner {
    tx: Sender<TaskLifecycle>,
}

impl Actor for TaskRunner {}

impl TaskRunner {
    pub fn new() -> Self {
        let (tx, mut rx): (Sender<TaskLifecycle>, Receiver<TaskLifecycle>) = mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(lifecycle) = rx.recv().await {
                info!(
                    "Received a new task {:?} with id {}.",
                    lifecycle.task, lifecycle.task.task_id
                );

                match lifecycle.task.kind.clone() {
                    TaskKind::SetupWorkspace {
                        location,
                        workspace,
                    } => {
                        info!(
                            "Received a request to setup a workspace: {:?}/{:?}.",
                            location, workspace
                        );

                        let host_actor = HostActor::from_registry().await.unwrap();
                        let database_actor = DatabaseActor::from_registry().await.unwrap();

                        lifecycle.init().await;

                        if let Err(e) = create_workspace(location).await {
                            lifecycle
                                .error(&format!("Failed to create workspace: {}", e.to_string()))
                                .await;
                            return;
                        };

                        lifecycle.progress("Created project directory.").await;

                        database_actor
                            .call(MigrateWorkspace {
                                workspace: workspace.to_string(),
                            })
                            .await
                            .unwrap()
                            .unwrap();

                        lifecycle.progress("Migrated database.").await;

                        host_actor
                            .call(EnableWorkspace { workspace })
                            .await
                            .unwrap()
                            .unwrap();

                        lifecycle.finish().await;
                    }

                    TaskKind::RemoveLocation { location, .. } => {
                        info!("Received a request to remove a location: {:?}.", location,);

                        lifecycle.init().await;

                        if let Err(e) = remove_location(location).await {
                            lifecycle
                                .error(&format!(
                                    "Failed to remove workspace directory: {}",
                                    e.to_string()
                                ))
                                .await;
                            return;
                        };

                        lifecycle.progress("Removed project directory.").await;

                        lifecycle.finish().await;
                    }

                    TaskKind::RunProcess {
                        workspace,
                        process_name,
                    } => {
                        info!(
                            "Received a request to run a process: {} -> {}.",
                            workspace.slug, process_name
                        );

                        lifecycle.init().await;

                        if let Err(e) = run_data_process(workspace, &process_name).await {
                            lifecycle
                                .error(&format!("Failed to run process: {}", e.to_string()))
                                .await;
                            return;
                        };

                        lifecycle
                            .progress(&format!("Finished process {}", process_name))
                            .await;

                        lifecycle.finish().await;
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
        let lifecycle = TaskLifecycle::new(msg.task);
        lifecycle.queued().await;
        self.tx.send(lifecycle).await?;
        Ok(())
    }
}
