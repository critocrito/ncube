use ncubed::{Application, ApplicationConfig};
use std::thread;
use tokio::runtime;
use tracing::Level;
use tracing_subscriber;
use web_view::*;

fn main() {
    // FIXME: supply config from command args/environment/config file
    let config = ApplicationConfig {
        host_db: "sqlite://ncube.db".into(),
        listen: "127.0.0.1:40666".parse().unwrap(),
    };

    let local_listen = config.listen.clone();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    thread::spawn(move || {
        let mut rt = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let mut ncube = Application::new(config);
            ncube.run().await.unwrap();
        });
    });

    web_view::builder()
        .title("Ncube")
        .content(Content::Url(format!("http://{}/index.html", local_listen)))
        .size(1024, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
