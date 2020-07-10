//! When connecting to remote ncube installation all requests are done using
//! HTTP. Internally the HTTP endpoint is treated like a database.
use bytes::buf::BufExt as _;
use chrono::{DateTime, Duration, Utc};
use hyper::{client::HttpConnector, Body, Client, Method, Request, StatusCode};
use ncube_data::Workspace;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, instrument};
use url::Url;

use crate::actors::{host::RequirePool, HostActor, Registry};
use crate::errors::{ActorError, StoreError};
use crate::http::{ErrorResponse, HttpResponse, SuccessResponse};
use crate::stores::account_store;
use crate::types::{LoginRequest, LoginResponse};

#[derive(Error, Debug)]
pub struct HttpConfigError;

impl Display for HttpConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpConfigError")
    }
}

#[instrument]
async fn login(url: &Url, email: &str, password: &str) -> Result<HttpAuth, StoreError> {
    let payload = serde_json::to_string(&LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    })
    .unwrap()
    .into_bytes();

    let req = Request::post(url.as_str())
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let client = Client::new();
    let res = client.request(req).await?;
    let status = res.status();
    let body = hyper::body::aggregate(res).await?;

    if status == StatusCode::CREATED {
        let data: SuccessResponse<LoginResponse> = serde_json::from_reader(body.reader())?;
        debug!("login success {:?}", data);
        let token = data.data.token;
        let created_at = Utc::now();
        Ok(HttpAuth { token, created_at })
    } else {
        let data: ErrorResponse = serde_json::from_reader(body.reader())?;
        debug!("login failed {:?}", data);
        Err(StoreError::HttpFail(data))
    }
}

#[derive(Debug, Clone)]
pub struct HttpAuth {
    token: String,
    created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    pub email: String,
    pub password: String,
    pub workspace: Workspace,
    pub url: Url,
    client: ClientWrapper,
    auth: Arc<RwLock<Option<HttpAuth>>>,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email && self.url.as_str() == other.url.as_str()
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Http::Database({:?})", self.url.as_str())
    }
}

impl Database {
    /// Construct a HTTP client.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ncube_data::Workspace;
    /// # use ncubed::db::http;
    /// # use url::Url;
    /// #
    /// # #[tokio::main]
    /// # async fn main () {
    /// let workspace = Workspace::default();
    /// let endpoint = Url::parse("http://example.org").unwrap();
    ///
    /// let db = http::Database::new(endpoint, &workspace, "my-email", "my-password");
    ///
    /// // Run a query on the connection object.
    /// # }
    /// ```
    pub fn new(endpoint: Url, workspace: &Workspace, email: &str, password: &str) -> Self {
        let client = Client::new();

        Self {
            client: ClientWrapper::new(client),
            auth: Arc::new(RwLock::new(None)),
            email: email.to_string(),
            password: password.to_string(),
            workspace: workspace.clone(),
            url: endpoint,
        }
    }

    pub async fn update_password(&mut self) -> Result<(), ActorError> {
        let mut host_actor = HostActor::from_registry().await.unwrap();
        let db = host_actor.call(RequirePool).await??;
        let account_store = account_store(db);
        let password = account_store
            .show_password(&self.email, &self.workspace)
            .await?;

        self.password = password;

        Ok(())
    }

    async fn execute<T>(&self, req: Request<Body>) -> Result<HttpResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned + Debug,
    {
        let res = self.client.request(req).await?;
        let status = res.status();
        let body = hyper::body::aggregate(res).await?;

        if status.is_success() {
            if status == StatusCode::OK || status == StatusCode::CREATED {
                let data: SuccessResponse<T> = serde_json::from_reader(body.reader())?;
                Ok(HttpResponse::Success(data))
            } else {
                Ok(HttpResponse::Empty)
            }
        } else {
            let data: ErrorResponse = serde_json::from_reader(body.reader())?;

            debug!("request failed: {:?}", data);

            Ok(HttpResponse::Error(data))
        }
    }

    pub(crate) async fn ensure_login(&self) -> Result<(), StoreError> {
        {
            let lock = self.auth.read().await;

            if let Some(auth) = &*lock {
                let now = Utc::now();
                // JWT tokens are valid for an hour, we have 5 minutes as a
                // buffer before we renew.
                let expire = now - Duration::minutes(55);

                if expire <= auth.created_at {
                    debug!("valid login found");
                    // We have a valid authentication token we can use.
                    return Ok(());
                }
            }
            // reader lock dropped so that we can acquire a write lock if we
            // have to update the authentication details.
        }

        self.login().await?;
        Ok(())
    }

    pub(crate) async fn login(&self) -> Result<(), StoreError> {
        let mut lock = self.auth.write().await;
        let mut url = self.url.clone();
        url.set_path(&format!("/api/workspaces/{}/account", self.workspace.slug));
        let http_auth = login(&url, &self.email, &self.password).await?;
        *lock = Some(http_auth);
        Ok(())
    }

    #[instrument]
    pub(crate) async fn get<T>(&self, url: Url) -> Result<Option<T>, StoreError>
    where
        T: serde::de::DeserializeOwned + Debug,
    {
        self.ensure_login().await?;
        let lock = self.auth.read().await;
        let req = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header("content-type", "application/json");
        let req = match &*lock {
            None => req,
            Some(auth) => req.header("authorization", format!("Bearer {}", auth.token)),
        };
        let req = req.body(Default::default()).unwrap();

        debug!("HTTP GET ({:?})", url.as_str());

        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(StoreError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn post<T, S>(&self, url: Url, payload: S) -> Result<Option<T>, StoreError>
    where
        T: serde::de::DeserializeOwned + Debug,
        S: serde::Serialize + Debug,
    {
        self.ensure_login().await?;
        let lock = self.auth.read().await;

        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();

        let req = Request::post(url.as_str()).header("content-type", "application/json");
        let req = match &*lock {
            None => req,
            Some(auth) => req.header("authorization", format!("Bearer {}", auth.token)),
        };
        let req = req.body(Body::from(payload_json)).unwrap();

        debug!("HTTP POST ({:?}) -> {:?}", url.as_str(), payload);

        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(StoreError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn put<T, S>(&self, url: Url, payload: S) -> Result<Option<T>, StoreError>
    where
        T: serde::de::DeserializeOwned + Debug,
        S: serde::Serialize + Debug,
    {
        self.ensure_login().await?;

        let lock = self.auth.read().await;

        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();

        let req = Request::put(url.as_str()).header("content-type", "application/json");
        let req = match &*lock {
            None => req,
            Some(auth) => req.header("authorization", format!("Bearer {}", auth.token)),
        };
        let req = req.body(Body::from(payload_json)).unwrap();

        debug!("HTTP PUT ({:?}) -> {:?}", url.as_str(), payload);

        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(StoreError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn delete(&self, url: Url) -> Result<(), StoreError> {
        self.ensure_login().await?;
        let lock = self.auth.read().await;

        let req = Request::delete(url.as_str()).header("content-type", "application/json");
        let req = match &*lock {
            None => req,
            Some(auth) => req.header("authorization", format!("Bearer {}", auth.token)),
        };
        let req = req.body(Default::default()).unwrap();

        debug!("HTTP DELETE ({:?})", url.as_str());

        let result = self.execute::<()>(req).await?;

        match result {
            HttpResponse::Error(data) => Err(StoreError::HttpFail(data)),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientWrapper {
    client: Client<HttpConnector, Body>,
}

impl ClientWrapper {
    pub(crate) fn new(client: Client<HttpConnector, Body>) -> Self {
        Self { client }
    }
}

impl Deref for ClientWrapper {
    type Target = Client<HttpConnector, Body>;
    fn deref(&self) -> &Client<HttpConnector, Body> {
        &self.client
    }
}

impl DerefMut for ClientWrapper {
    fn deref_mut(&mut self) -> &mut Client<HttpConnector, Body> {
        &mut self.client
    }
}
