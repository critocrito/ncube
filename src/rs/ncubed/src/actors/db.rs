use async_trait::async_trait;
use ncube_data::WorkspaceDatabase;
use std::result::Result;
use tracing::debug;
use xactor::{message, Actor, Context, Handler};

use crate::actors::host::{HostActor, ShowWorkspace};
use crate::db::{sqlite, Database};
use crate::errors::{ActorError, StoreError};
use crate::registry::Registry;

pub(crate) struct DatabaseActor {
    sqlite_cache: sqlite::DatabaseCache,
}

impl Actor for DatabaseActor {}
impl Registry for DatabaseActor {}

impl DatabaseActor {
    pub fn new() -> Self {
        let sqlite_cache = sqlite::DatabaseCache::new();

        Self { sqlite_cache }
    }
}

#[message(result = "Result<Database, ActorError>")]
#[derive(Debug)]
pub(crate) struct LookupDatabase {
    pub workspace: String,
}

#[async_trait]
impl Handler<LookupDatabase> for DatabaseActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: LookupDatabase,
    ) -> Result<Database, ActorError> {
        let mut actor = HostActor::from_registry().await.unwrap();
        let workspace = actor
            .call(ShowWorkspace {
                slug: msg.workspace,
            })
            .await??;
        let connection_string = workspace.connection_string();

        // Check if the database pool exists in the cache and create one if not.
        match workspace.database {
            WorkspaceDatabase::Sqlite { .. } => {
                if self.sqlite_cache.has(&connection_string) {
                    debug!("Database `{}` retrieved from cache.", connection_string);
                    let db = self.sqlite_cache.get(&connection_string).unwrap();
                    Ok(Database::Sqlite(db))
                } else {
                    debug!("Database `{}` not in cache.", connection_string);

                    let cfg = connection_string
                        .parse::<sqlite::Config>()
                        .map_err(|e| ActorError::Store(StoreError::SqliteConfig(e)))?;
                    let db = sqlite::Database::new(cfg, 5);
                    self.sqlite_cache.put(&connection_string, db);

                    let cached_db = self.sqlite_cache.get(&connection_string).ok_or_else(|| {
                        ActorError::Store(StoreError::NotFound(connection_string))
                    })?;

                    Ok(Database::Sqlite(cached_db))
                }
            }
        }
    }
}
