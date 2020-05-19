// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// #![cfg_attr(test, deny(warnings))]

// I set all modules to pub in order to use them inside doc tests. This is not
// intended to be a proper crate but rather a standalone application.
pub mod actors;
pub mod db;
pub mod errors;
pub mod fs;
pub mod handlers;
pub mod registry;
pub mod routes;
pub mod stores;
pub mod types;

use std::net::SocketAddr;
use xactor::Actor;

use self::actors::{DatabaseActor, HostActor, TaskActor};
use self::registry::Registry;

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

    pub async fn run(&mut self) -> Result<(), errors::ApplicationError> {
        let host_actor = HostActor::new(&self.config.host_db)?.start().await;
        HostActor::register_once(host_actor).await;
        let task_actor = TaskActor::new()?.start().await;
        TaskActor::register_once(task_actor).await;
        let database_actor = DatabaseActor::new().start().await;
        DatabaseActor::register_once(database_actor).await;

        warp::serve(routes::router()).run(self.config.listen).await;

        Ok(())
    }
}
