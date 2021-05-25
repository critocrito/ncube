use ncube_errors::HostError;
use std::net::SocketAddr;
use tracing::{error, info};
use warp::{
    http::{Method, StatusCode},
    Filter,
};

mod config;
mod fs;
mod headers;
mod http;
mod investigation;
mod methodology;
mod process;
mod segment;
mod source;
mod source_tag;
mod stat;
mod unit;
mod user;
mod workspace;
mod ws;

fn with_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["content-type"])
}

pub async fn start_http_api(listen: SocketAddr) -> Result<(), HostError> {
    warp::serve(router()).run(listen).await;
    Ok(())
}

pub(crate) fn router(
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    let log = warp::log::custom(|info| {
        let method = info.method();
        let path = info.path();
        let status = info.status();
        let elapsed = info.elapsed();
        if status.as_u16() >= 400 {
            error!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        } else {
            info!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        }
    });

    assets()
        .or(api())
        .or(warp::path("health")
            .map(|| Ok(StatusCode::NO_CONTENT))
            .with(with_cors()))
        .or(ws())
        .recover(http::handle_rejection)
        .with(log)
}

pub(crate) fn assets() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("index.html")
        .map(|| include_str!("../../../../target/dist/index.html"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/html"))
        .or(warp::path("styles.css")
            .map(|| include_str!("../../../../target/dist/styles.css"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/css")))
        .or(warp::path("app.js")
            .map(|| include_str!("../../../../target/dist/app.js"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript")))
        .or(warp::path("styles.css.map")
            .map(|| include_str!("../../../../target/dist/styles.css.map"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/css")))
        .or(warp::path("app.js.map")
            .map(|| include_str!("../../../../target/dist/app.js.map"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript")))
}

pub(crate) fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            config::routes()
                .or(workspace::routes())
                .or(source::routes())
                .or(user::routes())
                .or(stat::routes())
                .or(unit::routes())
                .or(source_tag::routes())
                .or(segment::routes())
                .or(process::routes())
                .or(investigation::routes())
                .or(methodology::routes()),
        )
        .with(with_cors())
}

pub(crate) fn ws() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register_path = warp::path("register");
    let register = register_path
        .and(warp::post())
        .and_then(ws::register)
        .or(register_path
            .and(warp::delete())
            .and(warp::path::param())
            .and_then(ws::unregister));

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and_then(ws::ws);

    register.or(ws).with(with_cors())
}
