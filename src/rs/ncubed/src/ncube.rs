use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};
use anyhow::Result;
use futures::join;
use std::fmt;
use tokio::{self, sync::mpsc};
use warp::Filter;

use crate::filters;
use crate::services::ncube_store_service;

pub struct Ncube {
    pub cfg: Config,
}

impl Ncube {
    pub async fn new(cfg: Config) -> Result<Self> {
        Ok(Ncube { cfg })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut ncube_store = NcubeStoreSqlite::new(self.cfg.ncube_db_path.clone()).await?;
        ncube_store.upgrade()?;

        let (tx, rx) = mpsc::channel(100);
        let ncube_store = ncube_store_service(rx, ncube_store);
        let static_assets = warp::get()
            .and(warp::path::end())
            .and(warp::fs::file("./public/index.html"));
        let routes = static_assets.or(warp::path!("api")
            .and(filters::ncube_config::routes(tx).recover(filters::handle_rejection)));
        let server = warp::serve(routes).run(([127, 0, 0, 1], 40666));

        let _ = join!(server, ncube_store);
        Ok(())
    }
}

impl fmt::Debug for Ncube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ncube").finish()
    }
}

#[derive(Debug)]
pub struct Config {
    pub ncube_db_path: String,
}
