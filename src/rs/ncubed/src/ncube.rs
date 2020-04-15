use anyhow::Result;
use std::fmt;
use tracing::info;
use warp::Filter;
use xactor::Actor;

use crate::actors::NcubeActor;
use crate::filters;
use crate::registry::Registry;
use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

pub struct Ncube {
    pub cfg: Config,
}

impl Ncube {
    pub async fn new(cfg: Config) -> Result<Self> {
        Ok(Ncube { cfg })
    }

    pub async fn run(&mut self) -> Result<()> {
        let log = warp::log::custom(|info| {
            let method = info.method();
            let path = info.path();
            let status = info.status();
            let elapsed = info.elapsed();
            info!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        });

        let mut ncube_store = NcubeStoreSqlite::new(self.cfg.ncube_db_path.clone()).await?;

        ncube_store.upgrade()?;

        let ncube_actor = NcubeActor::new(ncube_store).start().await;
        NcubeActor::register_once(ncube_actor).await;

        let static_assets = warp::get()
            .and(warp::path::end())
            .map(|| include_str!("../../../../resources/dist/index.html"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/html"));
        let routes = static_assets.or(warp::path!("api")
            .and(filters::ncube_config::routes().recover(filters::handle_rejection))
            .with(log));
        let _ = warp::serve(routes).run(([127, 0, 0, 1], 40666)).await;

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
