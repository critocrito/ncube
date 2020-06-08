use async_trait::async_trait;
use ncube_data::WorkspaceDatabase;
use std::result::Result;
use std::str::FromStr;
use tracing::debug;
use xactor::{message, Actor, Context, Handler};

use crate::actors::host::{HostActor, RequirePool};
use crate::cache::DatabaseCache;
use crate::db::{http, sqlite, Database};
use crate::errors::{ActorError, StoreError};
use crate::registry::Registry;
use crate::stores::{workspace_store, WorkspaceStore};

/// The database actor can be queried for database connections for workspaces.
/// Connection pools are cached when requested first time and subsequently
/// served from cache.
///
/// # Example
///
/// ```no_run
/// use ncubed::actors::db::{LookupDatabase, DatabaseActor};
/// use crate::ncubed::registry::Registry;
///
/// # #[tokio::main]
/// # async fn main () {
/// let mut database_actor = DatabaseActor::from_registry().await.unwrap();
///
/// let db = database_actor
///   .call(LookupDatabase {
///       workspace: "workspace".to_string(),
///     })
///     .await
///     .unwrap()
///     .unwrap();
///
/// # }
/// ```
#[derive(Debug)]
pub struct DatabaseActor {
    cache: DatabaseCache,
}

impl Actor for DatabaseActor {}
impl Registry for DatabaseActor {}

impl DatabaseActor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for DatabaseActor {
    fn default() -> Self {
        let cache = DatabaseCache::new();

        Self { cache }
    }
}

#[message(result = "Result<Database, ActorError>")]
#[derive(Debug)]
pub struct LookupDatabase {
    pub workspace: String,
}

#[async_trait]
impl Handler<LookupDatabase> for DatabaseActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: LookupDatabase,
    ) -> Result<Database, ActorError> {
        let mut host_actor = HostActor::from_registry().await.unwrap();

        let db = host_actor.call(RequirePool).await??;
        let workspace_store = workspace_store(db.clone());
        let workspace = workspace_store.show_by_slug(&msg.workspace).await?;

        let connection_string = workspace.connection_string();

        if self.cache.has(&connection_string) {
            debug!("Database `{}` retrieved from cache.", connection_string);
            let db = self.cache.get(&connection_string).unwrap();
            return Ok(db);
        }

        debug!("Database `{}` not in cache.", connection_string);

        let db = match workspace.database {
            WorkspaceDatabase::Sqlite { .. } => {
                let db = sqlite::Database::from_str(&connection_string, 5)
                    .map_err(|e| ActorError::Store(StoreError::SqliteConfig(e)))?;
                Database::Sqlite(Box::new(db))
            }
            WorkspaceDatabase::Http { .. } => {
                let db = http::Database::from_str(&connection_string)
                    .map_err(|e| ActorError::Store(StoreError::HttpConfig(e)))?;
                Database::Http(Box::new(db))
            }
        };

        self.cache.put(&connection_string, db);

        let cached_db = self
            .cache
            .get(&connection_string)
            .ok_or_else(|| ActorError::Store(StoreError::NotFound(connection_string)))?;

        Ok(cached_db)
    }
}
