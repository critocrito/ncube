use anyhow::Result;

use crate::stores::{sqlite::NcubeStoreSqlite, NcubeStore};

pub struct Ncube {
    pub ncube_store: Box<dyn NcubeStore>,
}

impl Ncube {
    pub async fn new(cfg: Config) -> Result<Self> {
        let ncube_store = NcubeStoreSqlite::new(cfg.ncube_db_path).await?;
        Ok(Ncube {
            ncube_store: Box::new(ncube_store),
        })
    }
}

pub struct Config {
    pub ncube_db_path: String,
}
