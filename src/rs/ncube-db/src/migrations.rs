use rusqlite::Connection;

use crate::errors::DatabaseError;

mod host {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub fn migrate_host(conn: &mut Connection) -> Result<(), DatabaseError> {
    host::migrations::runner().run(conn)?;
    Ok(())
}
