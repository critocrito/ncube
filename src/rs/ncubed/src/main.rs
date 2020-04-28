use ncubed::ncube::{Config, Ncube};
use tokio;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "sqlite://ncube.db".into(),
    };

    tracing_subscriber::fmt()
        // FIXME: make the log level configureable
        .with_max_level(Level::TRACE)
        .init();

    let mut ncube = Ncube::new(config);
    ncube.run().await.unwrap();
}
