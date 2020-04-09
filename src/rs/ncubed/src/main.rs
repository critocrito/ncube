use anyhow::Result;
use futures::join;
use serde::Serialize;
use std::convert::Infallible;
use tokio::{
    self,
    sync::mpsc::{self, error::SendError, Receiver, Sender},
    sync::oneshot,
};
use warp::{http::StatusCode, reply, Filter};

mod errors;
mod stores;
use self::errors::{DataStoreError, RouteRejection};
use self::stores::{sqlite::NcubeStoreSqlite, NcubeStore};
use ncube_data::NcubeConfig;

fn with_tx(tx: Sender<()>) -> impl Filter<Extract = (Sender<()>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

fn with_ncube_store(
    tx: Sender<NcubeStoreCmd>,
) -> impl Filter<Extract = (Sender<NcubeStoreCmd>,), Error = Infallible> + Clone {
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

trait ChannelCommand {}

#[derive(Debug)]
enum NcubeStoreCmd {
    IsBootstrapped(oneshot::Sender<Result<bool, DataStoreError>>),
    ShowConfig(oneshot::Sender<Result<NcubeConfig, DataStoreError>>),
}

impl ChannelCommand for NcubeStoreCmd {}

async fn wait_msgs(mut rx: Receiver<NcubeStoreCmd>, mut ncube: Ncube) -> Result<()> {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            NcubeStoreCmd::IsBootstrapped(tx) => match ncube.ncube_store.is_bootstrapped().await {
                Ok(is_bootstrapped) => {
                    println!("Bootstrapped: {}", is_bootstrapped);
                    let _ = tx.send(Ok(is_bootstrapped));
                }
                Err(err) => {
                    let _ = tx.send(Err(err));
                }
            },
            NcubeStoreCmd::ShowConfig(tx) => match ncube.ncube_store.show().await {
                Ok(config) => {
                    let _ = tx.send(Ok(config));
                }
                Err(err) => {
                    let _ = tx.send(Err(err));
                }
            },
        }
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

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(RouteRejection::ChannelError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "channel error";
    } else if let Some(RouteRejection::DataError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "data error";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

impl warp::reject::Reject for RouteRejection {}

impl From<RouteRejection> for warp::Rejection {
    fn from(rejection: RouteRejection) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}

impl From<SendError<NcubeStoreCmd>> for RouteRejection {
    fn from(_: SendError<NcubeStoreCmd>) -> RouteRejection {
        RouteRejection::ChannelError
    }
}

impl From<DataStoreError> for RouteRejection {
    fn from(_: DataStoreError) -> RouteRejection {
        RouteRejection::DataError
    }
}

async fn handler(mut s: Sender<NcubeStoreCmd>) -> Result<impl warp::Reply, warp::Rejection> {
    let (tx, rx) = oneshot::channel();

    let _ = s
        .send(NcubeStoreCmd::IsBootstrapped(tx))
        .await
        .map_err(|_| RouteRejection::ChannelError)?;

    if let Ok(false) = rx.await.map_err(|_| RouteRejection::ChannelError)? {
        return Err(warp::reject::not_found());
    }

    let (tx, rx) = oneshot::channel();

    let _ = s
        .send(NcubeStoreCmd::ShowConfig(tx))
        .await
        .map_err(|_| RouteRejection::ChannelError)?;

    let config = rx
        .await
        .map_err(|_| RouteRejection::ChannelError)?
        .map_err(|_| RouteRejection::DataError)?;

    Ok(reply::json(&config))
}

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    let ncube = run(config).await.unwrap();

    let (tx, rx) = mpsc::channel(1);
    let (tx1, rx1) = mpsc::channel(1);

    let (tx2, rx2) = mpsc::channel(100);

    let fix_later = wait_msgs(rx2, ncube);

    let static_assets = warp::path!("index.html").and(warp::fs::file("./public/index.html"));

    let ncube_config = warp::get()
        .and(warp::path::end())
        .and(with_ncube_store(tx2))
        .and_then(handler);

    let mgmt = warp::path!("quit")
        .and(with_tx(tx))
        .and(with_tx(tx1))
        .and_then(empty_reply);

    let routes = static_assets.or(ncube_config).recover(handle_rejection);

    let (_addr, data_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 40666), quit_signal(rx));

    let (_addr, mgmt_server) =
        warp::serve(mgmt).bind_with_graceful_shutdown(([127, 0, 0, 1], 40667), quit_signal(rx1));

    let _ = join!(data_server, mgmt_server, fix_later);
}
