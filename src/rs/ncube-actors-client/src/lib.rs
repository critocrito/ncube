use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ncube_actors_common::{message, Actor, ActorError, Context, Handler, Registry};
use ncube_cache::GuardedCache;
use ncube_data::Client;
use serde::Serialize;
use std::fmt::Debug;
use std::result::Result;
use tracing::error;
use uuid::Uuid;

pub trait PushNotification {}

#[derive(Debug, Serialize)]
pub struct PushNotificationEnvelope<T>
where
    T: Debug + Serialize + PushNotification + Send,
{
    created_at: DateTime<Utc>,
    #[serde(flatten)]
    data: T,
}

impl<T> PushNotificationEnvelope<T>
where
    T: Debug + Serialize + PushNotification + Send,
{
    pub fn new(data: T) -> Self {
        let now = Utc::now();

        PushNotificationEnvelope {
            created_at: now,
            data,
        }
    }
}

pub struct ClientActor {
    clients: GuardedCache<Client>,
}

impl Actor for ClientActor {}

impl Registry for ClientActor {}

impl ClientActor {
    pub fn new() -> Self {
        let clients: GuardedCache<Client> = GuardedCache::new();

        Self { clients }
    }
}

#[message(result = "Result<String, ActorError>")]
#[derive(Debug)]
pub struct RegisterClient {
    pub client_id: usize,
}

#[async_trait]
impl Handler<RegisterClient> for ClientActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RegisterClient,
    ) -> Result<String, ActorError> {
        let uuid = Uuid::new_v4()
            .to_simple()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();

        self.clients.put(
            &uuid,
            Client {
                client_id: msg.client_id,
                topics: vec![String::from("host")],
                sender: None,
            },
        );

        Ok(uuid)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct UnregisterClient {
    pub uuid: String,
}

#[async_trait]
impl Handler<UnregisterClient> for ClientActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UnregisterClient,
    ) -> Result<(), ActorError> {
        self.clients.delete(&msg.uuid);

        Ok(())
    }
}

#[message(result = "Result<Option<Client>, ActorError>")]
#[derive(Debug)]
pub struct ClientSubscription {
    pub uuid: String,
}

#[async_trait]
impl Handler<ClientSubscription> for ClientActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ClientSubscription,
    ) -> Result<Option<Client>, ActorError> {
        let client = self.clients.get(&msg.uuid);

        Ok(client)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct UpdateSubscription {
    pub uuid: String,
    pub client: Client,
}

#[async_trait]
impl Handler<UpdateSubscription> for ClientActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UpdateSubscription,
    ) -> Result<(), ActorError> {
        self.clients.reset(&msg.uuid, msg.client);

        Ok(())
    }
}

#[derive(Debug)]
pub struct PublishMessage<T>
where
    T: 'static + Send + Serialize + Debug + PushNotification,
{
    pub msg: T,
}

impl<T> xactor::Message for PublishMessage<T>
where
    T: Send + Serialize + Debug + PushNotification,
{
    type Result = Result<(), ActorError>;
}

#[async_trait]
impl<T> Handler<PublishMessage<T>> for ClientActor
where
    T: Send + Serialize + Debug + PushNotification,
{
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: PublishMessage<T>,
    ) -> Result<(), ActorError> {
        let envelope = PushNotificationEnvelope::new(msg.msg);
        let message = serde_json::to_string(&envelope).unwrap();

        for client in self.clients.entries().iter() {
            if let Some(channel) = &client.1.sender {
                if let Err(e) = channel.send(Ok(warp::ws::Message::text(&message))) {
                    error!(
                        "Error sending message to client {}: {}",
                        client.0,
                        e.to_string()
                    );
                }
            }
        }

        Ok(())
    }
}
