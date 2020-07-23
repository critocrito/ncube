use rusqlite::Connection;

use crate::errors::DatabaseError;

mod host {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

mod workspace {
    use refinery::embed_migrations;
    embed_migrations!("migrations-workspace");
}

pub fn migrate_host(conn: &mut Connection) -> Result<(), DatabaseError> {
    host::migrations::runner().run(conn)?;
    Ok(())
}

pub fn migrate_workspace(conn: &mut Connection) -> Result<(), DatabaseError> {
    workspace::migrations::runner().run(conn)?;
    Ok(())
}
