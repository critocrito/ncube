use ncubed::{Config, Ncube};
use std::thread;
use tokio::runtime;
use web_view::*;

fn main() {
    let port: i32 = 40666;

    // FIXME: supply config from command args/environment/config file
    let config = Config {
        ncube_db_path: "ncube.db".into(),
    };

    thread::spawn(move || {
        let mut rt = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let mut ncube = Ncube::new(config).await.unwrap();
            ncube.run().await.unwrap();
        });
    });

    web_view::builder()
        .title("Ncube")
        .content(Content::Url(format!("http://127.0.0.1:{}", port)))
        .size(1024, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
