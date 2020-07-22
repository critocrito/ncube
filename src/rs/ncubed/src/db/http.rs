//! When connecting to remote ncube installation all requests are done using
//! HTTP. Internally the HTTP endpoint is treated like a database.
use chrono::{DateTime, Duration, Utc};
use ncube_data::{
    ErrorResponse, HttpResponse, LoginRequest, LoginResponse, SuccessResponse, Workspace,
};
use reqwest::{Client, StatusCode};
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, instrument};
use url::Url;

use crate::db::errors::DatabaseError;

#[instrument]
async fn login(url: &Url, email: &str, password: &str) -> Result<HttpAuth, DatabaseError> {
    let client = Client::new();
    let resp = client
        .post(url.as_str())
        .header("content-type", "application/json")
        .json(&LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;
    let status = resp.status();

    if status == StatusCode::CREATED {
        let data: SuccessResponse<LoginResponse> = resp.json().await?;
        debug!("login success {:?}", data);
        let token = data.data.token;
        let created_at = Utc::now();
        Ok(HttpAuth { token, created_at })
    } else {
        let data: ErrorResponse = resp.json().await?;
        debug!("login failed {:?}", data);
        Err(DatabaseError::HttpFail(data))
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
    client: reqwest::Client,
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
        let client = reqwest::Client::new();

        Self {
            client,
            auth: Arc::new(RwLock::new(None)),
            email: email.to_string(),
            password: password.to_string(),
            workspace: workspace.clone(),
            url: endpoint,
        }
    }

    async fn execute<T>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<HttpResponse<T>, DatabaseError>
    where
        T: serde::de::DeserializeOwned + Debug,
    {
        self.ensure_login().await?;
        let lock = self.auth.read().await;

        let resp = match &*lock {
            None => req,
            Some(auth) => req.bearer_auth(&auth.token),
        }
        .send()
        .await?;

        let status = resp.status();

        if status.is_success() {
            if status == StatusCode::OK || status == StatusCode::CREATED {
                match resp.json().await {
                    Err(_) => Ok(HttpResponse::Empty),
                    Ok(data) => Ok(HttpResponse::Success(data)),
                }
            } else {
                Ok(HttpResponse::Empty)
            }
        } else {
            let data: ErrorResponse = resp.json().await?;

            debug!("request failed: {:?}", data);

            Ok(HttpResponse::Error(data))
        }
    }

    pub(crate) async fn ensure_login(&self) -> Result<(), DatabaseError> {
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

    pub(crate) async fn login(&self) -> Result<(), DatabaseError> {
        let mut lock = self.auth.write().await;
        let mut url = self.url.clone();
        url.set_path(&format!("/api/workspaces/{}/account", self.workspace.slug));
        let http_auth = login(&url, &self.email, &self.password).await?;
        *lock = Some(http_auth);
        Ok(())
    }

    #[instrument]
    pub(crate) async fn get<T>(&self, url: Url) -> Result<Option<T>, DatabaseError>
    where
        T: serde::de::DeserializeOwned + Debug,
    {
        debug!("HTTP GET ({:?})", url.as_str());

        let req = self.client.get(url.as_str());
        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(DatabaseError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn post<T, S>(&self, url: Url, payload: S) -> Result<Option<T>, DatabaseError>
    where
        T: serde::de::DeserializeOwned + Debug,
        S: serde::Serialize + Debug,
    {
        debug!("HTTP POST ({:?}) -> {:?}", url.as_str(), payload);

        let req = self.client.post(url.as_str()).json(&payload);
        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(DatabaseError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn put<T, S>(&self, url: Url, payload: S) -> Result<Option<T>, DatabaseError>
    where
        T: serde::de::DeserializeOwned + Debug,
        S: serde::Serialize + Debug,
    {
        debug!("HTTP PUT ({:?}) -> {:?}", url.as_str(), payload);

        let req = self.client.put(url.as_str()).json(&payload);
        let result = self.execute::<T>(req).await?;

        match result {
            HttpResponse::Success(data) => Ok(Some(data.data)),
            HttpResponse::Empty => Ok(None),
            HttpResponse::Error(data) => Err(DatabaseError::HttpFail(data)),
        }
    }

    #[instrument]
    pub(crate) async fn delete(&self, url: Url) -> Result<(), DatabaseError> {
        debug!("HTTP DELETE ({:?})", url.as_str());

        let req = self.client.delete(url.as_str());
        let result = self.execute::<()>(req).await?;

        match result {
            HttpResponse::Error(data) => Err(DatabaseError::HttpFail(data)),
            _ => Ok(()),
        }
    }
}
