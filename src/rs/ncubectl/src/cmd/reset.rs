use ncubed::{crypto, handlers};
use rand;

use crate::fatal;

pub(crate) async fn secret() {
    let rng = rand::thread_rng();
    let seed = crypto::mkpass(rng);
    let key = crypto::gen_secret_key(&seed);
    handlers::config::insert_config_setting("secret_key", &key)
        .await
        .unwrap_or_else(|e| fatal!("failed to reset secret key: {}", e.to_string()));
}
