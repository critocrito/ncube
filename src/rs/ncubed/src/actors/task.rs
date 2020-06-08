use async_trait::async_trait;
use tokio::sync::mpsc::{self, Sender};
use xactor::{message, Actor, Context, Handler};

use crate::actors::Registry;
use crate::errors::{ActorError, HostError};

#[derive(Debug)]
enum TaskMessage {
    SetupWorkspace(String),
}

pub(crate) struct TaskActor {
    tx: Sender<TaskMessage>,
}

mod tasks {
    use std::fmt::Debug;
    use std::path::Path;
    use tokio::process::Command;
    use tracing::{debug, info, instrument};

    use crate::errors::HostError;
    use crate::fs::{expand_tilde, mkdirp, unzip_workspace};

    #[instrument]
    pub(crate) async fn create_workspace<P: AsRef<Path> + Debug>(
        location: P,
    ) -> Result<(), HostError> {
        let expanded_path = expand_tilde(location)
            .ok_or_else(|| HostError::General("Failed to expand path".into()))?;

        mkdirp(&expanded_path)?;
        unzip_workspace(&expanded_path)?;

        let env_path = vec![
            "dist/nodejs/bin",
            "dist/ffmpeg",
            "dist/youtube-dl",
            "node_modules/.bin:/usr/local/bin:/usr/bin:/bin",
        ]
        .iter()
        .map(|s| format!("{}/{}", expanded_path.as_path().to_string_lossy(), s))
        .collect::<Vec<String>>()
        .join(":");

        debug!("PATH={}", env_path);

        Command::new("npm")
            .current_dir(expanded_path.clone())
            .env("PATH", &env_path)
            .arg("i")
            .spawn()
            .expect("npm failed to start")
            .await
            .expect("npm failed to run");

        info!("Installed Sugarcube dependencies.",);

        Command::new("sugarcube")
            .current_dir(expanded_path.clone())
            .env("PATH", &env_path)
            .arg("-p")
            .arg("sql_schema_migrate")
            .spawn()
            .expect("npm failed to start")
            .await
            .expect("npm failed to run");

        info!("Migrated the Sqlite database.",);

        Ok(())
    }
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
                        tasks::create_workspace(location)
                            .await
                            .expect("Failed to create workspace");
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
