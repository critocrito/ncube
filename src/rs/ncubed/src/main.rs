use ncubed::ncube::{Config, Ncube};
use tokio;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    tracing_subscriber::fmt()
        // FIXME: make the log level configureable
        .with_max_level(Level::DEBUG)
        .init();

    let mut ncube = Ncube::new(config).await.unwrap();
    ncube.run().await.unwrap();
}
