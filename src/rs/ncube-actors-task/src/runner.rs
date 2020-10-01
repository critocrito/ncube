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
    tx: Sender<(String, Task)>,
}

impl Actor for TaskRunner {}

impl TaskRunner {
    pub fn new() -> Self {
        let (tx, mut rx): (Sender<(String, Task)>, Receiver<(String, Task)>) = mpsc::channel(100);

        tokio::spawn(async move {
            while let Some((task_id, task)) = rx.recv().await {
                info!("Received a new task {:?} with id {}.", task, task_id);

                let lifecycle = TaskLifecycle::new(task.clone());

                match task.kind.clone() {
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

                        create_workspace(location)
                            .await
                            .expect("Failed to create workspace");

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

                        remove_location(location)
                            .await
                            .expect("Failed to remove a location");

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

                        run_data_process(workspace, &process_name)
                            .await
                            .expect("Failed to run process");

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
        self.tx.send((msg.task.task_id(), msg.task)).await?;
        Ok(())
    }
}
