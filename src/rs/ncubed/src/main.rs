use ncubed::{Application, ApplicationConfig};
use tracing::Level;

#[tokio::main]
async fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = ApplicationConfig {
        host_db: "sqlite://ncube.db".into(),
        listen: "127.0.0.1:40666".parse().unwrap(),
    };

    tracing_subscriber::fmt()
        // FIXME: make the log level configureable
        .with_max_level(Level::DEBUG)
        .init();

    let app = Application::new(config);
    app.run().await.unwrap();
}
