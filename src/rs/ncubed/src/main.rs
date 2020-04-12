use tokio;

use ncubed::ncube::{Config, Ncube};

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    let mut ncube = Ncube::new(config).await.unwrap();
    ncube.run().await.unwrap();
}
