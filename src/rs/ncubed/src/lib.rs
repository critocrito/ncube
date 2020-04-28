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

use std::fmt;
use std::net::SocketAddr;
use xactor::Actor;

use crate::registry::Registry;
use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

#[derive(Debug, Clone)]
pub struct ApplicationConfig {
    pub host_db: String,
    pub listen: SocketAddr,
}

pub struct Application {
    config: ApplicationConfig,
}

impl Application {
    pub fn new(config: ApplicationConfig) -> Self {
        Application { config }
    }

    pub async fn run(&mut self) -> Result<(), errors::ApplicationError> {
        let mut ncube_store = NcubeStoreSqlite::new(self.config.host_db.clone()).await?;

        ncube_store.upgrade().await?;

        let ncube_actor = actors::NcubeHost::new(ncube_store).start().await;
        actors::NcubeHost::register_once(ncube_actor).await;

        warp::serve(routes::router()).run(self.config.listen).await;

        Ok(())
    }
}

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ncube").finish()
    }
}
