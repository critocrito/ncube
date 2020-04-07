use anyhow::Result;
use futures::{join, Future};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio::{
    self,
    sync::mpsc::{self, Receiver, Sender},
};
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

async fn wait_msgs(mut rx: Receiver<()>, mut ncube: Ncube) -> Result<()> {
    while let Some(i) = rx.recv().await {
        async {
            println!("got = {:?}", i);
        }
        .await;
        let _collections = ncube.ncube_store.list_collections().await?;
    }
    Ok(())
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

async fn run(cfg: Config) -> Result<Ncube> {
    let mut ncube = Ncube::new(cfg).await?;
    ncube.ncube_store.upgrade()?;

    Ok(ncube)
}

#[derive(Debug, Serialize, Deserialize)]
struct NcubeConfig {
    workspace_root: String,
    name: String,
    email: String,
}

async fn handler(mut s: Sender<()>) -> Result<impl warp::Reply, Infallible> {
    match s.send(()).await {
        Ok(_) => println!("Send the damn thing."),

        Err(_) => println!("Error the damn thing."),
    }
    let cfg = NcubeConfig {
        workspace_root: "haha".to_string(),
        name: "huhu".to_string(),
        email: "heheh".to_string(),
    };
    Ok(warp::reply::json(&cfg))
}

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    let mut ncube = run(config).await.unwrap();

    let (tx, rx) = mpsc::channel(1);
    let (tx1, rx1) = mpsc::channel(1);

    let (mut tx2, mut rx2) = mpsc::channel(100);

    let xx = wait_msgs(rx2, ncube); // tokio::spawn(move || wait_msgs(rx2, ncube));

    let static_assets = warp::path!("index.html").and(warp::fs::file("./public/index.html"));

    let ncube_config = warp::get()
        .and(warp::path::end())
        .and(with_tx(tx2))
        .and_then(handler);

    let mgmt = warp::path!("quit")
        .and(with_tx(tx))
        .and(with_tx(tx1))
        .and_then(empty_reply);

    let routes = static_assets.or(ncube_config);

    let (_addr, data_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 40666), quit_signal(rx));

    let (_addr, mgmt_server) =
        warp::serve(mgmt).bind_with_graceful_shutdown(([127, 0, 0, 1], 40667), quit_signal(rx1));

    join!(xx, data_server, mgmt_server);
}
