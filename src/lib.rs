#![warn(missing_docs)]
//! Mactor - Market Actor
//! Framework for working with actors in a Trading setting,
//! if that even makes any difference, 
//! but heads up that it will be changed to my needs with that background.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    /// The Actors [`ActorHandle`]
    type Handle;


    type Context;

    //TODO: should return a result?
    fn spawn(self, context: Self::Context) -> Self::Handle;
}

/// Implemented on a handle for an [`Actor`].
// The implementor can implement other methods aswell
/// for extended functionality.
///
///
pub trait Handle where Self: Sized {
    fn shutdown(&mut self);
    #[allow(async_fn_in_trait)]
    async fn is_alive(&mut self) -> bool;
}

pub trait Sender<T>: Handle {
    #[allow(async_fn_in_trait)]
    async fn send(&mut self, message: T);
}

pub trait Reciever<T>: Handle {
    #[allow(async_fn_in_trait)]
    async fn recv(&mut self) -> Option<T>;
}
/// JoinAble
pub trait Joinable<T>:Handle {
    #[allow(async_fn_in_trait)]
    async fn join(self) -> Result<T> {
        unimplemented!()
    }
    
}
