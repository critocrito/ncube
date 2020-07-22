use async_trait::async_trait;
use ncube_errors::HostError;
use ncube_tasks::create_workspace;
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Sender};
use xactor::{message, Actor, Context, Handler};

use crate::{ActorError, Registry};

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
                match res {
                    TaskMessage::SetupWorkspace(location, workspace) => {
                        create_workspace(location)
                            .await
                            .expect("Failed to create workspace");

                        // FIXME: Can I handle the workspace migrations cleaner?
                        // Move the search indices into the migrations? Maybe a
                        // specialized message to the host actor?
                        // Generate the search indices and triggers for this workspace.
                        let mut database_actor = DatabaseActor::from_registry().await.unwrap();
                        let database = database_actor
                            .call(LookupDatabase { workspace })
                            .await
                            .unwrap()
                            .unwrap();
                        let search_store = search_store(database);
                        search_store.unit_index().await.unwrap();
                        search_store.source_index().await.unwrap();
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
