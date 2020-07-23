use async_trait::async_trait;
use ncube_data::{Account, WorkspaceDatabase};
use ncube_db::{errors::DatabaseError, http, migrations, sqlite, Database, DatabaseCache};
use ncube_stores::{account_store, workspace_store, WorkspaceStore};
use std::result::Result;
use tracing::{debug, info};
use url::Url;
use xactor::{message, Actor, Context, Handler};

use crate::{
    host::{HostActor, RequirePool},
    ActorError, Registry,
};

/// The database actor can be queried for database connections for workspaces.
/// Connection pools are cached when requested first time and subsequently
/// served from cache.
///
/// # Example
///
/// ```no_run
/// use ncube_actors::{db::{LookupDatabase, DatabaseActor}, Registry};
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

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct ResetDatabase {
    pub workspace: String,
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct MigrateWorkspace {
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
                    .map_err(|e| ActorError::Database(DatabaseError::SqliteConfig(e)))?;
                Database::Sqlite(Box::new(db))
            }
            WorkspaceDatabase::Http { .. } => {
                let account_store = account_store(db);
                let Account { email, .. } = account_store.show_by_workspace(&workspace).await?;
                let password = account_store.show_password(&email, &workspace).await?;
                let endpoint = Url::parse(&connection_string)
                    .map_err(|e| ActorError::Database(DatabaseError::HttpConfig(e.to_string())))?;

                let db = http::Database::new(endpoint, &workspace, &email, &password);

                Database::Http(Box::new(db))
            }
        };

        self.cache.put(&connection_string, db);

        let cached_db = self
            .cache
            .get(&connection_string)
            .ok_or_else(|| ActorError::Database(DatabaseError::NotFound(connection_string)))?;

        Ok(cached_db)
    }
}

#[async_trait]
impl Handler<ResetDatabase> for DatabaseActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: ResetDatabase) -> Result<(), ActorError> {
        let mut host_actor = HostActor::from_registry().await.unwrap();

        let db = host_actor.call(RequirePool).await??;
        let workspace_store = workspace_store(db.clone());
        let workspace = workspace_store.show_by_slug(&msg.workspace).await?;

        let connection_string = workspace.connection_string();

        if !self.cache.has(&connection_string) {
            debug!("Database `{}` not in cache.", connection_string);
            return Ok(());
        }

        match workspace.database {
            WorkspaceDatabase::Sqlite { .. } => Ok(()),
            WorkspaceDatabase::Http { .. } => {
                let account_store = account_store(db);
                let Account { email, .. } = account_store.show_by_workspace(&workspace).await?;
                let password = account_store.show_password(&email, &workspace).await?;
                let endpoint = Url::parse(&connection_string)
                    .map_err(|e| ActorError::Database(DatabaseError::HttpConfig(e.to_string())))?;

                let db = http::Database::new(endpoint, &workspace, &email, &password);

                self.cache
                    .reset(&connection_string, Database::Http(Box::new(db)));

                Ok(())
            }
        }
    }
}

#[async_trait]
impl Handler<MigrateWorkspace> for DatabaseActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: MigrateWorkspace,
    ) -> Result<(), ActorError> {
        let mut host_actor = HostActor::from_registry().await.unwrap();
        let db = host_actor.call(RequirePool).await??;
        let workspace_store = workspace_store(db);
        let workspace = workspace_store.show_by_slug(&msg.workspace).await?;

        let connection_string = workspace.connection_string();
        info!("connection_string {:?}", connection_string);

        match workspace.database {
            WorkspaceDatabase::Sqlite { .. } => {
                info!("matched the db");
                let db = sqlite::Database::from_str(&connection_string, 1)
                    .map_err(|e| ActorError::Database(DatabaseError::SqliteConfig(e)))?;
                let mut conn = db
                    .connection()
                    .await
                    .map_err(|e| DatabaseError::SqlitePool(e))?;
                migrations::migrate_workspace(&mut **conn)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
