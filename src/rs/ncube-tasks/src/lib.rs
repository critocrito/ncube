use ncube_cache::GuardedCache;
use ncube_data::{Task, Workspace, WorkspaceKind};
use ncube_errors::HostError;
use ncube_fs::{expand_tilde, mkdirp, unzip_workspace};
use remove_dir_all::remove_dir_all;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::{process::Command, sync::mpsc::Sender};
use tracing::{debug, info, instrument};

pub type TaskCache = GuardedCache<Task>;

#[instrument]
fn env_path(workspace_path: &PathBuf) -> String {
    vec![
        "dist/nodejs/bin",
        "dist/ffmpeg",
        "dist/youtube-dl",
        "node_modules/.bin:/usr/local/bin:/usr/bin:/bin",
    ]
    .iter()
    .map(|s| format!("{}/{}", workspace_path.as_path().to_string_lossy(), s))
    .collect::<Vec<String>>()
    .join(":")
}

#[instrument]
pub async fn create_workspace<P: AsRef<Path> + Debug>(
    location: P,
    tx: &mut Sender<String>,
) -> Result<(), HostError> {
    let expanded_path =
        expand_tilde(location).ok_or_else(|| HostError::General("Failed to expand path".into()))?;

    tx.send("Extracting the workspace from template.".to_string())
        .await
        .map_err(|e| {
            HostError::General(format!("Failed to send notification: {}", e.to_string()))
        })?;

    mkdirp(&expanded_path)?;
    unzip_workspace(&expanded_path)?;

    tx.send("Installing workspace dependencies.".to_string())
        .await
        .map_err(|e| {
            HostError::General(format!("Failed to send notification: {}", e.to_string()))
        })?;

    let env_path = env_path(&expanded_path);

    debug!("PATH={}", env_path);

    Command::new("npm")
        .current_dir(expanded_path.clone())
        .env("PATH", &env_path)
        .env("SUGARCUBE_SKIP_APACHE_TIKA_DOWNLOAD", "true")
        .arg("i")
        .spawn()
        .expect("npm failed to start")
        .await
        .expect("npm failed to run");

    tx.send("Setting up the workspace database.".to_string())
        .await
        .map_err(|e| {
            HostError::General(format!("Failed to send notification: {}", e.to_string()))
        })?;

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

#[instrument]
pub async fn remove_location<P: AsRef<Path> + Debug>(
    location: P,
    tx: &mut Sender<String>,
) -> Result<(), HostError> {
    let expanded_path =
        expand_tilde(location).ok_or_else(|| HostError::General("Failed to expand path".into()))?;

    tx.send("Removing the workspace directory.".to_string())
        .await
        .map_err(|e| {
            HostError::General(format!("Failed to send notification: {}", e.to_string()))
        })?;

    info!("Removing workspace directory {:?}", expanded_path);
    remove_dir_all(&expanded_path)?;

    Ok(())
}

#[instrument]
pub async fn run_data_process(
    workspace: Workspace,
    process_name: &str,
    tx: &mut Sender<String>,
) -> Result<(), HostError> {
    match workspace.kind {
        WorkspaceKind::Local(location) => {
            let expanded_path = expand_tilde(location)
                .ok_or_else(|| HostError::General("Failed to expand path".into()))?;

            let env_path = env_path(&expanded_path);

            debug!("PATH={}", env_path);

            let cmd = format!("processes/{}.sh", process_name);

            tx.send(format!("Running data process {}.", process_name))
                .await
                .map_err(|e| {
                    HostError::General(format!("Failed to send notification: {}", e.to_string()))
                })?;

            Command::new(&cmd)
                .current_dir(expanded_path.clone())
                .env("PATH", &env_path)
                .spawn()
                .expect(format!("data process {} failed to start", &cmd).as_str())
                .await
                .expect(format!("data process {} failed to run", &cmd).as_str());

            Ok(())
        }
        _ => Err(HostError::General(
            "Only local workspaces can run this task".into(),
        )),
    }
}
