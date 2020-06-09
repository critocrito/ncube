//! When connecting to remote ncube installation all requests are done using
//! HTTP. Internally the HTTP endpoint is treated like a database.
//!
//! # Example
//!
//! ```no_run
//! use ncubed::db::http;
//! use hyper::{Body, Method, Request};
//! # #[tokio::main]
//! # async fn main() {
//! let endpoint = "https://example.org";
//! let cfg = endpoint.parse::<http::Config>().unwrap();
//! let client = http::Database::new(cfg);
//! // client.get("workspaces")
//! // client.put("workspaces/1", ..)
//! // ..
//! # }
//! ```
use bytes::{buf::BufExt as _, Buf};
use chrono::{DateTime, Duration, Utc};
use hyper::{client::HttpConnector, Body, Client, Method, Request, Uri};
use serde_json::Value;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

use crate::errors::StoreError;
use crate::http::SuccessResponse;

#[derive(Error, Debug)]
pub struct HttpConfigError;

impl Display for HttpConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpConfigError")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) endpoint: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: Url::parse("http://127.0.0.1:40666").unwrap(),
        }
    }
}

impl FromStr for Config {
    type Err = HttpConfigError;

    fn from_str(s: &str) -> Result<Self, HttpConfigError> {
        let endpoint = Url::parse(s).map_err(|_| HttpConfigError)?;

        Ok(Config { endpoint })
    }
}

#[derive(Debug, Clone)]
pub struct HttpAuth {
    token: String,
    created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    config: Config,
    client: ClientWrapper,
    email: String,
    password: String,
    workspace: String,
    auth: Option<HttpAuth>,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Http::Database({:?})", self.config)
    }
}

impl Database {
    /// Construct a HTTP client.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ncubed::db::http;
    /// # #[tokio::main]
    /// # async fn main () {
    /// let config = "http://example.org".parse::<http::Config>().unwrap();
    /// let db = http::Database::new(config);
    /// // Run a query on the connection object.
    /// # }
    /// ```
    pub fn new(config: Config, workspace: &str, email: &str, password: &str) -> Self {
        let client = Client::new();

        Self {
            client: ClientWrapper::new(client),
            auth: None,
            email: email.to_string(),
            password: password.to_string(),
            workspace: workspace.to_string(),
            config,
        }
    }

    fn url(&self, path: &str) -> Uri {
        let mut uri = self.config.endpoint.clone();
        uri.set_path(path);
        Uri::from_str(uri.as_str()).unwrap()
    }

    async fn execute(&self, req: Request<Body>) -> Result<impl Buf, hyper::error::Error> {
        let res = self.client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;

        Ok(body)
    }

    #[allow(dead_code)]
    pub(crate) async fn login_if_needed(&mut self) -> Result<(), StoreError> {
        if let Some(auth) = &self.auth {
            let now = Utc::now();
            let expire = now - Duration::minutes(55);

            if expire <= auth.created_at {
                // We have a valid authentication token we can use.
                return Ok(());
            }
        }

        self.login().await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn login(&mut self) -> Result<(), StoreError> {
        let login_path = format!("/api/workspaces/{}/account", self.workspace);
        let url = self.url(&login_path);
        let payload = serde_json::json!({"email": self.email, "password": self.password});
        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();

        let req = Request::post(url)
            .header("content-type", "application/json")
            .body(Body::from(payload_json))
            .unwrap();
        let body = self.execute(req).await?;

        let data: Value = serde_json::from_reader(body.reader())?;
        let token = data["data"]["token"].to_string();
        let created_at = Utc::now();
        self.auth = Some(HttpAuth { token, created_at });

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn get<T>(&self, path: &str) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(&path);
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("content-type", "application/json")
            .body(Default::default())
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn post<T, S>(
        &self,
        path: &str,
        payload: S,
    ) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
        S: serde::Serialize,
    {
        let url = self.url(&path);
        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
        let req = Request::post(url)
            .header("content-type", "application/json")
            .body(Body::from(payload_json))
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn put<T, S>(
        &self,
        path: &str,
        payload: S,
    ) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
        S: serde::Serialize,
    {
        let url = self.url(&path);
        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
        let req = Request::put(url)
            .header("content-type", "application/json")
            .body(Body::from(payload_json))
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn delete<T, S>(&self, path: &str) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(&path);
        let req = Request::delete(url)
            .header("content-type", "application/json")
            .body(Default::default())
            .unwrap();

        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
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
