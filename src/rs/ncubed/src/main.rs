use anyhow::Result;
use futures::join;
use std::convert::Infallible;
use tokio::{
    self,
    sync::mpsc::{self, Receiver, Sender},
};
use warp::Filter;

mod errors;
mod filters;
mod handlers;
mod ncube;
mod services;
mod stores;

use self::filters::handle_rejection;
use self::ncube::{Config, Ncube};
use self::services::ncube_store_service;

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

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    let mut ncube = Ncube::new(config).await.unwrap();
    ncube.run().await.unwrap();

    let (tx, rx) = mpsc::channel(1);
    let (tx1, rx1) = mpsc::channel(1);

    let (tx2, rx2) = mpsc::channel(100);

    let ncube_store = ncube_store_service(rx2, ncube);

    let static_assets = warp::path!("index.html").and(warp::fs::file("./public/index.html"));

    let mgmt = warp::path!("quit")
        .and(with_tx(tx))
        .and(with_tx(tx1))
        .and_then(empty_reply);

    let routes = static_assets.or(filters::ncube_config::routes(tx2).recover(handle_rejection));

    let (_addr, data_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 40666), quit_signal(rx));

    let (_addr, mgmt_server) =
        warp::serve(mgmt).bind_with_graceful_shutdown(([127, 0, 0, 1], 40667), quit_signal(rx1));

    let _ = join!(data_server, mgmt_server, ncube_store);
}
