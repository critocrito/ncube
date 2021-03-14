use futures::{FutureExt, StreamExt};
use ncube_data::{Client, ClientSubscription, SuccessResponse};
use ncube_handlers::{host as handlers, HandlerError};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{error, info};
use warp::ws::WebSocket;

pub(crate) async fn register() -> Result<impl warp::Reply, warp::Rejection> {
    let uuid = handlers::register_client().await?;
    let subscription = ClientSubscription {
        url: format!("ws://127.0.0.1:40666/ws/{}", uuid),
        uuid,
    };
    let response = SuccessResponse::new(subscription);

    Ok(warp::reply::json(&response))
}

pub(crate) async fn unregister(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::unregister_client(&id).await?;

    Ok(warp::http::StatusCode::OK)
}

pub(crate) async fn ws(ws: warp::ws::Ws, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let client = handlers::client_subscription(&id).await?;

    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| async {
            // FIXME: Handle error in a non panicy way.
            client_connection(socket, id, c).await.unwrap();
        })),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn client_connection(
    ws: WebSocket,
    id: String,
    mut client: Client,
) -> Result<(), HandlerError> {
    let (client_ws_tx, _client_ws_rx) = ws.split();
    let (client_tx, client_rx) = mpsc::unbounded_channel();
    let client_rx = UnboundedReceiverStream::new(client_rx);

    info!("Connected client {} ({}).", client.client_id, id);

    tokio::task::spawn(client_rx.forward(client_ws_tx).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_tx);

    let _ = handlers::update_subscription(&id, client.clone()).await?;

    info!("Updated client subscription {}", id);

    Ok(())
}
