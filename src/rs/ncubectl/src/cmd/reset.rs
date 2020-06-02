use ncubed::{crypto, handlers};

use crate::fatal;

pub(crate) async fn secret() {
    let key = crypto::gen_secret_key();
    handlers::config::insert_config_setting("secret_key", &key)
        .await
        .unwrap_or_else(|e| fatal!("failed to reset secret key: {}", e.to_string()));
}
