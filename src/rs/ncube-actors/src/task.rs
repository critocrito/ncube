use async_trait::async_trait;
use ncube_errors::HostError;
use ncube_tasks::create_workspace;
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Sender};
use tracing::info;
use xactor::{message, Actor, Context, Handler};

use crate::{
    db::{DatabaseActor, MigrateWorkspace},
    ActorError, Registry,
};

#[derive(Debug)]
enum TaskMessage {
    SetupWorkspace(String, String),
}

pub struct TaskActor {
    tx: Sender<TaskMessage>,
}

impl Actor for TaskActor {}

impl Registry for TaskActor {}

impl TaskActor {
    pub fn new() -> Result<Self, HostError> {
        let (tx, mut rx) = mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                info!("Received a new task.");
                match res {
                    TaskMessage::SetupWorkspace(location, workspace) => {
                        info!(
                            "Received a request to setup a workspace: {:?}/{:?}.",
                            location, workspace
                        );

                        create_workspace(location)
                            .await
                            .expect("Failed to create workspace");

                        let mut database_actor = DatabaseActor::from_registry().await.unwrap();
                        database_actor
                            .call(MigrateWorkspace { workspace })
                            .await
                            .unwrap()
                            .unwrap();
                    }
                }
            }
        });

        Ok(TaskActor { tx })
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
        _ctx: &Context<Self>,
        msg: SetupWorkspace,
    ) -> Result<(), ActorError> {
        self.tx
            .send(TaskMessage::SetupWorkspace(msg.location, msg.workspace))
            .await?;
        Ok(())
    }
}
