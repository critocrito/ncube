use web_view::*;

fn main() {
    let port = 40666;

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
