use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::debug;
use warp::Filter;

use crate::crypto::jwt_verify;
use crate::errors::HostError;
use crate::handlers::config::show_secret_key;
use crate::types::ReqCtx;

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
            return false;
        })
        .boxed()
}

pub fn user_ctx() -> warp::filters::BoxedFilter<(Option<String>,)> {
    warp::header("authorization")
        .map(|header_value: String| {
            let parts: Vec<&str> = header_value.split(" ").collect();
            match &parts[..] {
                ["Bearer", token] => Some(token.to_string()),
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
        Ok(claims) => (claims.subject, claims.audience),
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

            match key {
                Ok(key) => {
                    let (email, workspace) = extract_claims(&key, &token);

                    return Ok(ReqCtx {
                        is_authorized: email.is_some() && workspace.is_some(),
                        is_local,
                        email,
                        workspace,
                    });
                }
                _ => {
                    return Ok(ReqCtx {
                        is_local,
                        ..Default::default()
                    })
                }
            };
            // I added this code branch to have Rust infer the return type of
            // and_then.
            #[allow(unreachable_code)]
            Err(warp::reject::custom(HostError::AuthError))
        },
    )
}

pub(crate) fn restrict_to_local_req(
) -> impl Filter<Extract = (ReqCtx,), Error = warp::Rejection> + Clone {
    authorize_req().and_then(move |ctx: ReqCtx| {
        debug!("{:?}", ctx);

        match ctx {
            ReqCtx { is_local: true, .. } => futures::future::ok(ctx),
            _ => futures::future::err(warp::reject::custom(HostError::AuthError)),
        }
    })
}

pub(crate) fn authenticate_remote_req(
) -> impl Filter<Extract = (ReqCtx,), Error = warp::Rejection> + Clone {
    authorize_req().and_then(move |ctx: ReqCtx| {
        debug!("{:?}", ctx);

        match ctx {
            ReqCtx {
                is_local: false,
                is_authorized: true,
                ..
            } => futures::future::err(warp::reject::custom(HostError::AuthError)),
            _ => futures::future::ok(ctx),
        }
    })
}
