use ncube_actors_client::{ClientActor, PublishMessage, PushNotification};
use ncube_actors_common::Registry;
use ncube_data::{Task, TaskKind, TaskState};
use serde::Serialize;
use std::fmt::Debug;

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
}

impl PushNotification for TaskPushNotification {}

struct TaskLifecycle {
    task: Task,
    task_id: String,
}

impl TaskLifecycle {
    fn new(task: Task) -> Self {
        let task_id = task.task_id();
        Self { task, task_id }
    }

    fn task_label(&self) -> String {
        match &self.task.kind {
            TaskKind::SetupWorkspace { .. } => "setup_workspace".to_string(),
            TaskKind::RemoveLocation { .. } => "remove_project".to_string(),
            TaskKind::RunProcess { process_name, .. } => format!("run_{}", process_name),
        }
    }

    fn topic(&self) -> String {
        format!("task.{}.{}", self.task.workspace, self.task_label(),)
    }

    async fn queued(&self) {
        let client_actor = ClientActor::from_registry().await.unwrap();

        client_actor
            .call(PublishMessage {
                topic: self.topic(),
                msg: TaskPushNotification {
                    event: NotificationEvent::Queued,
                    task_id: self.task_id.clone(),
                },
            })
            .await
            .unwrap()
            .unwrap();
    }

    // FIXME: Handle error
    async fn init(&self) {
        let task_actor = TaskActor::from_registry().await.unwrap();
        let client_actor = ClientActor::from_registry().await.unwrap();

        client_actor
            .call(PublishMessage {
                topic: self.topic(),
                msg: TaskPushNotification {
                    event: NotificationEvent::Start,
                    task_id: self.task_id.clone(),
                },
            })
            .await
            .unwrap()
            .unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id.clone(),
                state: TaskState::Running,
            })
            .await
            .unwrap()
            .unwrap();
    }

    async fn progress(&self, msg: &str) {
        let client_actor = ClientActor::from_registry().await.unwrap();

        client_actor
            .call(PublishMessage {
                topic: self.topic(),
                msg: TaskPushNotification {
                    event: NotificationEvent::Progress {
                        msg: msg.to_string(),
                    },
                    task_id: self.task_id.clone(),
                },
            })
            .await
            .unwrap()
            .unwrap();
    }

    async fn finish(&self) {
        let task_actor = TaskActor::from_registry().await.unwrap();
        let client_actor = ClientActor::from_registry().await.unwrap();

        client_actor
            .call(PublishMessage {
                topic: self.topic(),
                msg: TaskPushNotification {
                    event: NotificationEvent::Done,
                    task_id: self.task_id.clone(),
                },
            })
            .await
            .unwrap()
            .unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id.clone(),
                state: TaskState::Done,
            })
            .await
            .unwrap()
            .unwrap();
    }

    async fn error(&self, msg: &str) {
        let task_actor = TaskActor::from_registry().await.unwrap();
        let client_actor = ClientActor::from_registry().await.unwrap();

        client_actor
            .call(PublishMessage {
                topic: self.topic(),
                msg: TaskPushNotification {
                    event: NotificationEvent::Error {
                        error: msg.to_string(),
                    },
                    task_id: self.task_id.clone(),
                },
            })
            .await
            .unwrap()
            .unwrap();

        task_actor
            .call(UpdateTask {
                task_id: self.task_id.clone(),
                state: TaskState::Failed(msg.to_string()),
            })
            .await
            .unwrap()
            .unwrap();
    }
}
