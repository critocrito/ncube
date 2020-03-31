use anyhow::Result;
use futures::join;
use std::convert::Infallible;
use tokio::sync::mpsc::{self, Receiver, Sender};
use warp::Filter;

use ncubed::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

fn with_tx(tx: Sender<()>) -> impl Filter<Extract = (Sender<()>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

async fn empty_reply(
    mut tx: Sender<()>,
    mut tx1: Sender<()>,
) -> Result<impl warp::Reply, Infallible> {
    tx.send(()).await.unwrap();
    tx1.send(()).await.unwrap();
    Ok(warp::reply())
}

async fn quit_signal(mut rx: Receiver<()>) {
    rx.recv().await.unwrap();
    println!("Quitting!");
}

struct Ncube {
    ncube_store: Box<dyn NcubeStore>,
}

impl Ncube {
    async fn new(cfg: Config) -> Result<Self> {
        let ncube_store = NcubeStoreSqlite::new(cfg.ncube_db_path).await?;
        Ok(Ncube {
            ncube_store: Box::new(ncube_store),
        })
    }
}

struct Config {
    ncube_db_path: String,
}

async fn run(cfg: Config) -> Result<()> {
    let mut ncube = Ncube::new(cfg).await?;
    ncube.ncube_store.upgrade()?;
    let num = ncube.ncube_store.show_number().await?;
    println!("{}", num);
    Ok(())
}

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    run(config).await.unwrap();

    let (tx, rx) = mpsc::channel(1);
    let (tx1, rx1) = mpsc::channel(1);

    let routes = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./public/index.html"));

    let mgmt = warp::path!("quit")
        .and(with_tx(tx))
        .and(with_tx(tx1))
        .and_then(empty_reply);

    let (_addr, data_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 40666), quit_signal(rx));

    let (_addr, mgmt_server) =
        warp::serve(mgmt).bind_with_graceful_shutdown(([127, 0, 0, 1], 40667), quit_signal(rx1));

    join!(data_server, mgmt_server);
}
