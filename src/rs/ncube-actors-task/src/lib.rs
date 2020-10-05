use ncube_actors_client::{ClientActor, PublishMessage, PushNotification};
use ncube_actors_common::Registry;
use ncube_data::{Task, TaskKind, TaskState};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

mod runner;
mod task;

pub use self::runner::*;
pub use self::task::*;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NotificationEvent {
    Queued,
    Start,
    Progress { msg: String },
    Done,
    Error { error: String },
}

#[derive(Debug, Serialize)]
pub struct TaskPushNotification {
    #[serde(flatten)]
    event: NotificationEvent,
    task_id: String,
    workspace: String,
    label: String,
    order: usize,
}

impl PushNotification for TaskPushNotification {}

struct TaskLifecycle {
    task: Task,
    msg_order: AtomicUsize,
}

impl TaskLifecycle {
    fn new(task: Task) -> Self {
        Self {
            task,
            msg_order: AtomicUsize::new(0),
        }
    }

    fn label(&self) -> String {
        match &self.task.kind {
            TaskKind::SetupWorkspace { .. } => "setup_workspace".to_string(),
            TaskKind::RemoveLocation { .. } => "remove_project".to_string(),
            TaskKind::RunProcess { process_name, .. } => format!("run_{}", process_name),
        }
    }

    fn task_id(&self) -> String {
        self.task.task_id().clone()
    }

    fn workspace(&self) -> String {
        self.task.workspace.clone()
    }

    async fn push_to_client(&self, event: NotificationEvent) {
        let client_actor = ClientActor::from_registry().await.unwrap();
        let order = self.msg_order.fetch_add(1, Ordering::Relaxed);

        client_actor
            .call(PublishMessage {
                msg: TaskPushNotification {
                    task_id: self.task_id(),
                    label: self.label(),
                    workspace: self.workspace(),
                    event,
                    order,
                },
            })
            .await
            .unwrap()
            .unwrap();
    }

    async fn queued(&self) {
        self.push_to_client(NotificationEvent::Queued).await;
    }

    // FIXME: Handle error
    async fn init(&self) {
        let task_actor = TaskActor::from_registry().await.unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id(),
                state: TaskState::Running,
            })
            .await
            .unwrap()
            .unwrap();

        self.push_to_client(NotificationEvent::Start).await;
    }

    async fn progress(&self, msg: &str) {
        self.push_to_client(NotificationEvent::Progress {
            msg: msg.to_string(),
        })
        .await;
    }

    async fn finish(&self) {
        let task_actor = TaskActor::from_registry().await.unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id(),
                state: TaskState::Done,
            })
            .await
            .unwrap()
            .unwrap();

        self.push_to_client(NotificationEvent::Done).await;
    }

    async fn error(&self, msg: &str) {
        let task_actor = TaskActor::from_registry().await.unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id(),
                state: TaskState::Failed(msg.to_string()),
            })
            .await
            .unwrap()
            .unwrap();

        self.push_to_client(NotificationEvent::Error {
            error: msg.to_string(),
        })
        .await;
    }
}
