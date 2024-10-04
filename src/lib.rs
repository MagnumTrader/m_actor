#![warn(missing_docs)]
//! Mactor - Market Actor
//! Framework for working with actors in a Trading setting,
//! if that even makes any difference, 
//! but heads up that it will be changed to my needs with that background.
//!

mod error;
pub use error::Error;

type Result<T> = std::result::Result<T, crate::Error>;


/// The [`Actor`] trait is used to enforce spawning the task,
/// and return a Handle which the user can assume  to be able to communicate with the [`Actor`].
///
/// if the latter, we need to give out the handle before.
/// i was thinking that could be a builder pattern. but i can get back to that later
/// check out [this](obsidian://open?vault=Notes&file=Bon%20-%20Builder%20Crate)
pub trait Actor
where
    Self::Handle: Handle,
{
    type Handle;

    type Context;

    fn spawn(self, context: Self::Context) -> Self::Handle;
}

/// The `Handle` trait defines a basic handle for controlling an actor. It
/// includes the ability to shut down the actor and check if it is still
/// alive.
///
/// # Example
/// ```ignore
/// struct MyHandle {
///     handle: JoinHandle<()>
/// };
///
/// impl Handle for MyHandle {
///     fn shutdown(&mut self) {
///         // handle shutdown logic here
///         self.handle.abort();
///     }
///     
///     async fn is_alive(&mut self) -> bool {
///         true
///     }
/// }
/// ```
///
/// [`Actor`] // for extended functionality.
pub trait Handle where Self: Sized {
    fn shutdown(&mut self);
    #[allow(async_fn_in_trait)]
    async fn is_alive(&mut self) -> bool;
}


/// The `Sender<T>` trait extends a [`Handle`] implementor.
/// It includes functionality to send messages to the [`Actor`] with the intention
/// if altering its functionality and 
///
/// Actors communicate via messages.
///
/// # Examples
///```ignore
/// struct MyHandle {
///     handle: JoinHandle<()>
///     tx: tokio::sync::mpsc::Sender<MyActorMessage>
/// };
///
/// enum MyActorMessage {
///     DoSomethingImportant,
///     GiveMeSomethingBack {response: ()} // Oneshot channels fits well here
///     Shutdown,
/// }
///
/// impl Handle for MyHandle {
///     fn shutdown(&mut self) {
///         // handle shutdown logic here
///         self.handle.abort();
///     }
///     
///     async fn is_alive(&mut self) -> bool {
///         true
///     }
/// }
///
/// impl Sender<MyActorMessage> for MyHandle {
///     async fn send(&mut self, message: MyActorMessage) {
///        let _ = self.tx.send(message).await;
///     }
/// }
///```

pub trait Sender<T>: Handle {
    #[allow(async_fn_in_trait)]
    async fn send(&mut self, message: T);
}

pub trait Reciever<T>: Handle {
    #[allow(async_fn_in_trait)]
    async fn recv(&mut self) -> Option<T>;
}

pub trait Joinable<T>:Handle {
    #[allow(async_fn_in_trait)]
    async fn join(self) -> Result<T>;
    
}
