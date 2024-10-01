#![allow(unused, unreachable_code)]
//! Mactor - Market Actor
//! Framework for working with actors in a Trading setting
//!
//!
//! have a Actor trait that takes a M
//! ActorHandle trait


/// The actor trait is used to enforce spawning the task
/// should it be an async function that we spawn to any runtime.
/// or should we internally use tokio spawn and have a non async function?
///
/// if the latter, we need to give out the handle before.
/// i was thinking that could be a builder pattern. but i can get back to that later
/// check out [this](obsidian://open?vault=Notes&file=Bon%20-%20Builder%20Crate)
pub trait Actor<M>
where
    Self: Sized,
    Self::Handle: ActorHandle<M>,
{
    /// The Actors [`ActorHandle`]
    /// Used to communicate with the actor
    type Handle;

    /// Context needed for running this Actor
    /// use () if not relevant
    /// can use a wrapper type later if needed but context is good
    type Context;

    #[allow(async_fn_in_trait)]
    async fn run(self);
    fn spawn(self, context: Self::Context) -> Self::Handle;
}

/// Implemented on a handle for an [`Actor`].
/// The implementor can implement other methods aswell
/// for extended functionality.
pub trait ActorHandle<M> {
    fn shutdown(&mut self);
    #[allow(async_fn_in_trait)]
    async fn is_alive(&mut self) -> bool;
}

pub trait ActorSender<M>: ActorHandle<M> {
    #[allow(async_fn_in_trait)]
    async fn send(&mut self, message: M);
}
pub trait ActorReciever<M>: ActorHandle<M> {
    #[allow(async_fn_in_trait)]
    async fn recv(&mut self) -> M;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
