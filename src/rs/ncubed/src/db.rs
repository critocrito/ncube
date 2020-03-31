use refinery_migrations;
use rusqlite::{self, Connection};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("connection not established")]
    FailedConnection(#[from] rusqlite::Error),
    #[error("migration failed")]
    Migration(#[from] refinery_migrations::Error),
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub fn migrate(path: String) -> Result<(), DataStoreError> {
    let mut conn = Connection::open(path)?;
    embedded::migrations::runner().run(&mut conn)?;
    Ok(())
}
