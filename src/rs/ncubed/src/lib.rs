// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

use std::fmt;
use std::net::SocketAddr;
use xactor::Actor;

use crate::actors::NcubeActor;
use crate::registry::Registry;
use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

pub(crate) mod actors;
pub(crate) mod handlers;
pub(crate) mod messages;
pub(crate) mod registry;
pub(crate) mod types;

pub mod db;
pub mod errors;
pub mod stores;

mod routes;

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

    pub async fn run(&mut self) -> Result<(), crate::errors::ApplicationError> {
        let mut ncube_store = NcubeStoreSqlite::new(self.config.host_db.clone()).await?;

        ncube_store.upgrade().await?;

        let ncube_actor = NcubeActor::new(ncube_store).start().await;
        NcubeActor::register_once(ncube_actor).await;

        warp::serve(routes::router()).run(self.config.listen).await;

        Ok(())
    }
}

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ncube").finish()
    }
}
