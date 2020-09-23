use ncube_data::{ClientSubscription, SuccessResponse};
use ncube_handlers::host as handlers;

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
