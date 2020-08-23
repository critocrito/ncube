use ncube_data::{ReqCtx, SuccessResponse};
use ncube_handlers::config as handlers;
use serde::Deserialize;
use warp::Filter;

use crate::http::restrict_to_local_req;

#[derive(Debug, Deserialize)]
struct SettingRequest {
    name: String,
    value: String,
}

async fn show(_ctx: ReqCtx) -> Result<impl warp::Reply, warp::Rejection> {
    // To bypass the bootstrap screen for now, we will initialize Ncube
    // automatically with a default workspace root. This will also generate the
    // secret key.
    let config = match handlers::show_config().await {
        Err(_) => {
            handlers::bootstrap(vec![("workspace_root".to_string(), "~/Ncube".to_string())])
                .await?;
            handlers::show_config().await?
        }
        Ok(cfg) => cfg,
    };
    let response = SuccessResponse::new(config);

    Ok(warp::reply::json(&response))
}

async fn create(
    _ctx: ReqCtx,
    settings: Vec<SettingRequest>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut config = vec![];
    for SettingRequest { name, value } in settings {
        config.push((name, value));
    }

    handlers::bootstrap(config).await?;

    Ok(warp::reply())
}

async fn update(
    _ctx: ReqCtx,
    settings: Vec<SettingRequest>,
) -> Result<impl warp::Reply, warp::Rejection> {
    for SettingRequest { name, value } in settings {
        handlers::insert_config_setting(&name, &value).await?;
    }

    Ok(warp::reply())
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(restrict_to_local_req())
        .and_then(show)
        .or(warp::post()
            .and(warp::path::end())
            .and(restrict_to_local_req())
            .and(warp::body::json())
            .and_then(create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .map(|reply| warp::reply::with_header(reply, "location", "/")))
        .or(warp::put()
            .and(warp::path::end())
            .and(restrict_to_local_req())
            .and(warp::body::json())
            .and_then(update)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
}
