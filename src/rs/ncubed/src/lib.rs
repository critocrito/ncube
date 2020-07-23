// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![type_length_limit = "1343848"]
// #![cfg_attr(test, deny(warnings))]

// I set all modules to pub in order to use them inside doc tests. This is not
// intended to be a proper crate but rather a standalone application.
pub mod errors;
pub mod handlers;
pub mod http;
pub mod routes;

use ncube_actors::{Actor, DatabaseActor, HostActor, Registry, TaskActor};
use std::net::SocketAddr;

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

    async fn setup(&self) -> Result<(), errors::ApplicationError> {
        let host_actor = HostActor::new(&self.config.host_db)?.start().await;
        HostActor::register_once(host_actor).await;
        let task_actor = TaskActor::new()?.start().await;
        TaskActor::register_once(task_actor).await;
        let database_actor = DatabaseActor::new().start().await;
        DatabaseActor::register_once(database_actor).await;

        Ok(())
    }

    /// Run `ncubed` as a daemon. This will start an HTTP server as well.
    pub async fn run(&self) -> Result<(), errors::ApplicationError> {
        self.setup().await?;
        warp::serve(routes::router()).run(self.config.listen).await;

        Ok(())
    }

    /// Run `ncubed` without a HTTP server.
    pub async fn run_without_http(&self) -> Result<(), errors::ApplicationError> {
        self.setup().await?;
        Ok(())
    }
}
