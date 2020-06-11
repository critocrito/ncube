use std::fmt::Debug;
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, info, instrument};

use crate::errors::HostError;
use crate::fs::{expand_tilde, mkdirp, unzip_workspace};

#[instrument]
pub(crate) async fn create_workspace<P: AsRef<Path> + Debug>(location: P) -> Result<(), HostError> {
    let expanded_path =
        expand_tilde(location).ok_or_else(|| HostError::General("Failed to expand path".into()))?;

    mkdirp(&expanded_path)?;
    unzip_workspace(&expanded_path)?;

    let env_path = vec![
        "dist/nodejs/bin",
        "dist/ffmpeg",
        "dist/youtube-dl",
        "node_modules/.bin:/usr/local/bin:/usr/bin:/bin",
    ]
    .iter()
    .map(|s| format!("{}/{}", expanded_path.as_path().to_string_lossy(), s))
    .collect::<Vec<String>>()
    .join(":");

    debug!("PATH={}", env_path);

    Command::new("npm")
        .current_dir(expanded_path.clone())
        .env("PATH", &env_path)
        .arg("i")
        .spawn()
        .expect("npm failed to start")
        .await
        .expect("npm failed to run");

    info!("Installed Sugarcube dependencies.",);

    Command::new("sugarcube")
        .current_dir(expanded_path.clone())
        .env("PATH", &env_path)
        .arg("-p")
        .arg("sql_schema_migrate")
        .spawn()
        .expect("npm failed to start")
        .await
        .expect("npm failed to run");

    info!("Migrated the Sqlite database.",);

    Ok(())
}
