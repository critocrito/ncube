// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![type_length_limit = "1343848"]
// #![cfg_attr(test, deny(warnings))]

use ncube_actors::{Actor, ActorError, DatabaseActor, HostActor, Registry, TaskActor};
use ncube_db::errors::DatabaseError;
use ncube_errors::HostError;
use ncube_http_api::start_http_api;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Actor(#[from] ActorError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Host(#[from] HostError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ApplicationConfig {
    pub host_db: String,
    pub listen: SocketAddr,
}

#[derive(Debug)]
pub struct Application {
    config: ApplicationConfig,
}

impl Application {
    pub fn new(config: ApplicationConfig) -> Self {
        Application { config }
    }

    async fn setup(&self) -> Result<(), ApplicationError> {
        let host_actor = HostActor::new(&self.config.host_db)?.start().await?;
        HostActor::register_once(host_actor).await;
        let task_actor = TaskActor::new().await.start().await?;
        TaskActor::register_once(task_actor).await;
        let database_actor = DatabaseActor::new().start().await?;
        DatabaseActor::register_once(database_actor).await;

        Ok(())
    }

    /// Run `ncubed` as a daemon. This will start an HTTP server as well.
    pub async fn run(&self) -> Result<(), ApplicationError> {
        self.setup().await?;
        start_http_api(self.config.listen).await?;

        Ok(())
    }

    /// Run `ncubed` without a HTTP server.
    pub async fn run_without_http(&self) -> Result<(), ApplicationError> {
        self.setup().await?;
        Ok(())
    }
}
