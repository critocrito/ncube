// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

mod actors;
mod db;
mod errors;
mod handlers;
mod registry;
mod routes;
mod stores;

use std::net::SocketAddr;
use xactor::Actor;

use self::actors::HostActor;
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

        warp::serve(routes::router()).run(self.config.listen).await;

        Ok(())
    }
}
