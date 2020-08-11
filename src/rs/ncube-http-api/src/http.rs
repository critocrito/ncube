use ncube_crypto::jwt_verify;
use ncube_data::{ErrorResponse, ReqCtx};
use ncube_db::DatabaseError;
use ncube_errors::HostError;
use ncube_handlers::{config::show_secret_key, HandlerError};
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::{debug, error};
use warp::{http::StatusCode, Filter};

pub(crate) async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    error!("{:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(HandlerError::Invalid(reason)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = reason.into();
    } else if let Some(HandlerError::NotFound(reason)) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = reason.into();
    } else if let Some(HandlerError::NotAllowed(reason)) = err.find() {
        code = StatusCode::FORBIDDEN;
        message = reason.to_string();
    } else if let Some(HostError::AuthError(reason)) = err.find() {
        error!("{:?}", reason);
        code = StatusCode::UNAUTHORIZED;
        message = "request did not authorize".to_string();
    } else if let Some(DatabaseError::HttpFail(reason)) = err.find() {
        error!("{:?}", reason);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    } else if let Some(rejection) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = rejection.to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorResponse::new(code, &message));

    Ok(warp::reply::with_status(json, code))
}

fn is_loopback() -> warp::filters::BoxedFilter<(bool,)> {
    warp::header("x-forwarded-for")
        .map(|forwarded_for: String| IpAddr::from_str(&forwarded_for).ok())
        .or(warp::addr::remote().map(|remote: Option<SocketAddr>| remote.map(|socket| socket.ip())))
        .unify()
        .map(|remote: Option<IpAddr>| {
            if let Some(ip) = remote {
                if ip.is_loopback() {
                    return true;
                }
            }
            false
        })
        .boxed()
}

pub fn user_ctx() -> warp::filters::BoxedFilter<(Option<String>,)> {
    warp::header("authorization")
        .map(|header_value: String| {
            let parts: Vec<&str> = header_value.split(' ').collect();
            match &parts[..] {
                ["Bearer", token] => Some((*token).to_string()),
                _ => None,
            }
        })
        .or(warp::any().map(|| None))
        .unify()
        .boxed()
}

fn extract_claims(key: &str, token: &str) -> (Option<String>, Option<String>) {
    let claims = jwt_verify(&key, &token);
    match claims {
        Ok(claims) => (claims.audience, claims.subject),
        Err(_) => (None, None),
    }
}

pub(crate) fn authorize_req() -> impl Filter<Extract = (ReqCtx,), Error = warp::Rejection> + Clone {
    warp::any().and(is_loopback()).and(user_ctx()).and_then(
        move |is_local: bool, token: Option<String>| async move {
            let token = match token {
                None => {
                    return Ok(ReqCtx {
                        is_local,
                        ..Default::default()
                    })
                }
                Some(token) => token,
            };

            let key = show_secret_key().await;

            debug!("authorizing token {:?} using key {:?}", token, key);

            match key {
                Ok(key) => {
                    let (workspace, email) = extract_claims(&key, &token);

                    debug!("authorized claims: {:?}/{:?}", workspace, email);

                    return Ok(ReqCtx {
                        is_authorized: email.is_some() && workspace.is_some(),
                        is_local,
                        email,
                        workspace,
                    });
                }
                _ => {
                    debug!("no key to extract claims");

                    return Ok(ReqCtx {
                        is_local,
                        ..Default::default()
                    });
                }
            };

            // I added this code branch to have Rust infer the return type of
            // and_then.
            #[allow(unreachable_code)]
            Err(warp::reject::custom(HostError::AuthError(
                "Unreachable branch".into(),
            )))
        },
    )
}

pub(crate) fn restrict_to_local_req(
) -> impl Filter<Extract = (ReqCtx,), Error = warp::Rejection> + Clone {
    authorize_req().and_then(move |ctx: ReqCtx| {
        debug!("restrict to local request: {:?}", ctx);

        match ctx {
            ReqCtx { is_local: true, .. } => futures::future::ok(ctx),
            _ => futures::future::err(warp::reject::custom(HostError::AuthError(
                "request is not local".into(),
            ))),
        }
    })
}

pub(crate) fn authenticate_remote_req(
) -> impl Filter<Extract = (ReqCtx,), Error = warp::Rejection> + Clone {
    authorize_req().and_then(move |ctx: ReqCtx| {
        debug!("authenticate remote request: {:?}", ctx);

        match ctx {
            ReqCtx {
                is_local: false,
                is_authorized: false,
                ..
            } => futures::future::err(warp::reject::custom(HostError::AuthError(
                "remote request did not authorize".into(),
            ))),
            _ => futures::future::ok(ctx),
        }
    })
}
