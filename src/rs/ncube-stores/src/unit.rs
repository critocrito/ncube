use async_trait::async_trait;
use ncube_data::{Download, Media, QueryTag, Source, Unit};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, Error as RusqliteError};
use serde_rusqlite::from_rows;
use tracing::instrument;

pub fn unit_store(wrapped_db: Database) -> Box<dyn UnitStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(UnitStoreSqlite { db }),
        Database::Http(client) => Box::new(UnitStoreHttp { client }),
    }
}

#[async_trait]
pub trait UnitStore {
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Unit>, DatabaseError>;
    async fn show(&self, id: i32) -> Result<Option<Unit>, DatabaseError>;
}

#[derive(Debug)]
pub struct UnitStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl UnitStore for UnitStoreSqlite {
    #[instrument]
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Unit>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/unit/paginate.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/unit/list-media.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/unit/list-downloads.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/unit/list-sources.sql"))?;
        let mut stmt5 = conn.prepare_cached(include_str!("../sql/unit/list-tags.sql"))?;

        let offset = page * page_size;
        let mut units: Vec<Unit> = vec![];

        for unit in from_rows::<Unit>(stmt.query(params![offset as i32, page_size as i32])?) {
            let mut unit = unit?;
            let mut medias: Vec<Media> = vec![];
            let mut downloads: Vec<Download> = vec![];
            let mut sources: Vec<Source> = vec![];
            let mut tags: Vec<QueryTag> = vec![];

            for media in from_rows::<Media>(stmt2.query(params![unit.id])?) {
                medias.push(media?);
            }

            for download in from_rows::<Download>(stmt3.query(params![unit.id])?) {
                downloads.push(download?);
            }

            for source in from_rows::<Source>(stmt4.query(params![unit.id])?) {
                sources.push(source?);
            }

            for tag in from_rows::<QueryTag>(stmt5.query(params![unit.id])?) {
                tags.push(tag?);
            }

            unit.media = medias;
            unit.downloads = downloads;
            unit.sources = sources;
            unit.tags = tags;

            units.push(unit);
        }

        Ok(units)
    }

    #[instrument]
    async fn show(&self, id: i32) -> Result<Option<Unit>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/unit/show-by-id.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/unit/list-media.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/unit/list-downloads.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/unit/list-sources.sql"))?;
        let mut stmt5 = conn.prepare_cached(include_str!("../sql/unit/list-tags.sql"))?;

        let mut unit = match stmt.query_row(params![id], |row| {
            Ok(Unit {
                id: row.get(0)?,
                id_hash: row.get(1)?,
                content_hash: row.get(3)?,
                source: row.get(5)?,
                unit_id: row.get(6)?,
                body: row.get(7)?,
                href: row.get(8)?,
                author: row.get(9)?,
                title: row.get(10)?,
                description: row.get(11)?,
                language: row.get(12)?,
                created_at: row.get(13)?,
                fetched_at: row.get(14)?,
                // data: row.get(15)?,
                media: vec![],
                downloads: vec![],
                sources: vec![],
                tags: vec![],
            })
        }) {
            Ok(value) => value,
            Err(RusqliteError::QueryReturnedNoRows) => return Ok(None),
            Err(e) => Err(e)?,
        };

        let mut medias: Vec<Media> = vec![];
        let mut downloads: Vec<Download> = vec![];
        let mut sources: Vec<Source> = vec![];
        let mut tags: Vec<QueryTag> = vec![];

        for media in from_rows::<Media>(stmt2.query(params![unit.id])?) {
            medias.push(media?);
        }

        for download in from_rows::<Download>(stmt3.query(params![unit.id])?) {
            downloads.push(download?);
        }

        for source in from_rows::<Source>(stmt4.query(params![unit.id])?) {
            sources.push(source?);
        }

        for tag in from_rows::<QueryTag>(stmt5.query(params![unit.id])?) {
            tags.push(tag?);
        }

        unit.media = medias;
        unit.downloads = downloads;
        unit.sources = sources;
        unit.tags = tags;

        Ok(Some(unit))
    }
}

#[derive(Debug)]
pub struct UnitStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl UnitStore for UnitStoreHttp {
    #[instrument]
    async fn list(&self, page: i32, page_size: i32) -> Result<Vec<Unit>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/data",
            self.client.workspace.slug
        ));
        url.query_pairs_mut()
            .clear()
            .append_pair("page", &page.to_string())
            .append_pair("size", &page_size.to_string());

        let data: Vec<Unit> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    async fn show(&self, id: i32) -> Result<Option<Unit>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/data/units/{}",
            self.client.workspace.slug, id
        ));

        let data: Option<Unit> = self.client.get(url).await?;

        Ok(data)
    }
}
