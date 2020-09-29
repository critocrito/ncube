#![type_length_limit = "1358044"]

use directories::ProjectDirs;
use ncubed::{Application, ApplicationConfig};
use serde::Deserialize;
use std::fs::create_dir_all;
use std::thread;
use tokio::runtime;
use tracing::{info, Level};
use web_view::*;

/// The UI can communicate to the Rust host using message passing. The type of
/// messages are encoded in the `Cmd` enum.
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    /// Open an external URL in the default browser.
    Url { url: String },
}

fn main() {
    let project = ProjectDirs::from("net", "sugarcubetools", "Ncube").unwrap();
    let cfg_dir = project.config_dir();
    create_dir_all(&cfg_dir).unwrap();
    let db_path = cfg_dir.join("ncube.db");

    info!("Using {:?} as Ncube configuration.", db_path);

    let config = ApplicationConfig {
        // FIXME: Handle the Option.unwrap explicitely
        host_db: format!("sqlite://{}", db_path.to_str().unwrap()),
        listen: "127.0.0.1:40666".parse().unwrap(),
    };

    let local_listen = config.listen;

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
            let ncube = Application::new(config);
            ncube.run().await.unwrap();
        });
    });

    web_view::builder()
        .title("Ncube")
        .content(Content::Url(format!("http://{}/index.html", local_listen)))
        .size(1280, 1024)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            use Cmd::*;

            let _ = match serde_json::from_str(arg).unwrap() {
                Url { url } => webbrowser::open(&url),
            };

            Ok(())
        })
        .run()
        .unwrap();
}
