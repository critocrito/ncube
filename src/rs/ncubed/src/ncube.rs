use std::fmt;
use tracing::info;
use warp::{http::Method, Filter};
use xactor::Actor;

use crate::actors::NcubeActor;
use crate::errors::ActorError;
use crate::filters;
use crate::registry::Registry;
use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

pub struct Ncube {
    pub cfg: Config,
}

impl Ncube {
    pub fn new(cfg: Config) -> Self {
        Ncube { cfg }
    }

    pub async fn run(&mut self) -> Result<(), ActorError> {
        let log = warp::log::custom(|info| {
            let method = info.method();
            let path = info.path();
            let status = info.status();
            let elapsed = info.elapsed();
            info!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        });

        let mut ncube_store = NcubeStoreSqlite::new(self.cfg.ncube_db_path.clone()).await?;

        ncube_store.upgrade().await?;

        let ncube_actor = NcubeActor::new(ncube_store).start().await;
        NcubeActor::register_once(ncube_actor).await;

        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
            .allow_headers(vec!["content-type"]);

        let routes = warp::path("index.html")
            .map(|| include_str!("../../../../target/dist/index.html"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/html"))
            .or(warp::path("styles.css")
                .map(|| include_str!("../../../../target/dist/styles.css"))
                .map(|reply| warp::reply::with_header(reply, "content-type", "text/css")))
            .or(warp::path("app.js")
                .map(|| include_str!("../../../../target/dist/app.js"))
                .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript")))
            .or(warp::path!("fonts" / "NotoSans-Regular.ttf")
                .map(|| {
                    let file = include_bytes!("../../../../target/dist/fonts/NotoSans-Regular.ttf");
                    file.to_vec()
                })
                .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
            .or(warp::path!("fonts" / "NotoSans-Bold.ttf")
                .map(|| {
                    let file = include_bytes!("../../../../target/dist/fonts/NotoSans-Bold.ttf");
                    file.to_vec()
                })
                .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
            .or(warp::path("api").and(filters::ncube_config::routes().with(cors)))
            .recover(filters::handle_rejection)
            .with(log);

        warp::serve(routes).run(([127, 0, 0, 1], 40666)).await;

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
