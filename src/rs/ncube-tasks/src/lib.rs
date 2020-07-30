use ncube_cache::GuardedCache;
use ncube_data::{Workspace, WorkspaceKind};
use ncube_errors::HostError;
use ncube_fs::{expand_tilde, mkdirp, unzip_workspace};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::{debug, info, instrument};

#[derive(Debug, Clone)]
pub enum TaskKind {
    SetupWorkspace(String, String),
    RunProcess(Workspace, String),
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Queued,
    Running,
    Failed(String),
    Done,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub kind: TaskKind,
    pub state: TaskState,
}

impl Task {
    pub fn new(kind: TaskKind) -> Self {
        Self {
            kind,
            state: TaskState::Queued,
        }
    }

    pub fn workspace(location: &str, workspace: &str) -> Self {
        Task::new(TaskKind::SetupWorkspace(
            location.to_string(),
            workspace.to_string(),
        ))
    }

    pub fn data_process(workspace: &Workspace, key: &str) -> Self {
        Task::new(TaskKind::RunProcess(workspace.clone(), key.to_string()))
    }
}

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
pub async fn create_workspace<P: AsRef<Path> + Debug>(location: P) -> Result<(), HostError> {
    let expanded_path =
        expand_tilde(location).ok_or_else(|| HostError::General("Failed to expand path".into()))?;

    mkdirp(&expanded_path)?;
    unzip_workspace(&expanded_path)?;

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
pub async fn run_data_process(workspace: Workspace, key: &str) -> Result<(), HostError> {
    match workspace.kind {
        WorkspaceKind::Local(location) => {
            let expanded_path = expand_tilde(location)
                .ok_or_else(|| HostError::General("Failed to expand path".into()))?;

            let env_path = env_path(&expanded_path);

            debug!("PATH={}", env_path);

            let cmd = format!("processes/{}.sh", key);

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
