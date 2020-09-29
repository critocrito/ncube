use async_trait::async_trait;
use fnv::FnvHasher;
use futures::lock::Mutex;
use ncube_db::DatabaseError;
use ncube_errors::HostError;
use once_cell::sync::OnceCell;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use thiserror::Error;

pub use xactor::{message, Actor, Addr, Context, Handler};

#[derive(Error, Debug)]
pub enum ActorError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("The host gave an error: {0}")]
    Host(String),

    #[error("The request to the actor was invalid: {0}")]
    Invalid(String),
}

impl From<HostError> for ActorError {
    fn from(err: HostError) -> Self {
        ActorError::Host(err.to_string())
    }
}

impl From<anyhow::Error> for ActorError {
    fn from(err: anyhow::Error) -> Self {
        ActorError::Host(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ActorError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        ActorError::Host(err.to_string())
    }
}

type ActorRegistry = HashMap<TypeId, Box<dyn Any + Send>, BuildHasherDefault<FnvHasher>>;
static REGISTRY: OnceCell<Mutex<ActorRegistry>> = OnceCell::new();

/// Create a global registry for actors. This registry stores at most one instance of an actor.
///
/// # Examples
///
/// ```no_run
/// use xactor::Actor;
/// use ncube_actors_common::Registry;
///
/// # #[tokio::main]
/// # async fn main() {
/// struct MyActor;
///
/// impl Actor for MyActor {}
/// impl Registry for MyActor {}
///
/// let act = MyActor.start().await.unwrap();
/// MyActor::register_once(act).await;
///
/// let act_again = MyActor::from_registry().await.unwrap();
/// # }
/// ```
#[async_trait]
pub trait Registry: Actor {
    /// Add a new actor to the registry. Panic if the actor is already registered.
    ///
    /// # Panics
    ///
    /// Actors can only be registered once. This function panics if one tries to register the same actor twice.
    ///
    async fn register_once(addr: Addr<Self>) {
        let registry = REGISTRY.get_or_init(Default::default);
        let mut registry = registry.lock().await;

        match registry.get_mut(&TypeId::of::<Self>()) {
            Some(_addr) => panic!("Can register an actor only once."),
            None => {
                registry.insert(TypeId::of::<Self>(), Box::new(addr));
                drop(registry);
            }
        }
    }

    async fn from_registry() -> Option<Addr<Self>> {
        let registry = REGISTRY.get_or_init(Default::default);
        let mut registry = registry.lock().await;

        registry
            .get_mut(&TypeId::of::<Self>())
            .map(|addr| addr.downcast_ref::<Addr<Self>>().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn retrieves_the_same_actor_multiple_times() {
        struct MyActor;

        impl Actor for MyActor {};
        impl Registry for MyActor {};

        let act = MyActor.start().await.unwrap();

        MyActor::register_once(act).await;

        let act2 = MyActor::from_registry().await.unwrap();
        let act3 = MyActor::from_registry().await.unwrap();

        assert_eq!(act2.actor_id(), act3.actor_id());
    }

    #[tokio::test]
    #[should_panic(expected = "Can register an actor only once.")]
    async fn panics_when_registering_the_same_actor_twice() {
        struct MyActor;

        impl Actor for MyActor {};
        impl Registry for MyActor {};

        let act = MyActor.start().await.unwrap();
        let act2 = MyActor.start().await.unwrap();

        MyActor::register_once(act).await;
        MyActor::register_once(act2).await;
    }
}
