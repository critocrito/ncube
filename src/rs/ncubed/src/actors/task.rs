use async_trait::async_trait;
use tokio::{
    process::Command,
    sync::mpsc::{self, Sender},
};
use tracing::{debug, info};
use xactor::{message, Actor, Context, Handler};

use crate::errors::{ActorError, HostError};
use crate::fs::expand_tilde;
use crate::registry::Registry;

#[derive(Debug)]
enum TaskMessage {
    SetupWorkspace(String),
}

pub(crate) struct TaskActor {
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
                    TaskMessage::SetupWorkspace(location) => {
                        let expanded_path = expand_tilde(location)
                            .ok_or(HostError::General("Failed to expand path".into()))
                            .expect("Fail");

                        let env_path = format!(
                            "{}/dist/nodejs/bin:/bin:/usr/bin",
                            expanded_path.as_path().to_string_lossy(),
                        );

                        debug!("Running npm: {:?} ({})", expanded_path, env_path);

                        Command::new("npm")
                            .current_dir(expanded_path.clone())
                            .env("PATH", env_path)
                            .arg("i")
                            .spawn()
                            .expect("npm failed to start")
                            .await
                            .expect("npm failed to run");

                        info!(
                            "Installed Sugarcube dependencies in {}.",
                            expanded_path.as_path().to_string_lossy()
                        );
                    }
                }
            }
        });

        Ok(TaskActor { tx })
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub(crate) struct SetupWorkspace {
    pub(crate) location: String,
}

#[async_trait]
impl Handler<SetupWorkspace> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: SetupWorkspace,
    ) -> Result<(), ActorError> {
        self.tx
            .send(TaskMessage::SetupWorkspace(msg.location))
            .await?;
        Ok(())
    }
}
