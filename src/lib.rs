#![allow(unused, unreachable_code)]
//! Mactor - Market Actor
//! Framework for working with actors in a Trading setting,
//! if that even makes any difference,
//! but heads up that it will be changed to my needs with that background.
//!

static SETTINGS: Mutex<i32> = Mutex::new(0);

mod concurrency;
mod error;
use std::sync::Mutex;

pub use error::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub trait Sendable: Sized + Send + Sync + 'static {}
impl<T> Sendable for T where T: Sized + Send + Sync + 'static {}

/// The [`Actor`] trait is used to enforce spawning the task,
/// and return a Handle which the user can assume  to be able to communicate with the [`Actor`].
///
/// if the latter, we need to give out the handle before.
/// i was thinking that could be a builder pattern. but i can get back to that later
/// check out [this](obsidian://open?vault=Notes&file=Bon%20-%20Builder%20Crate)
pub trait Actor: Sized + Send + 'static {
    type Msg: Send;
    type Context: Send;
    type Arguments: Send;

    fn pre_start(arguments: Self::Arguments) -> Result<Self::Context>;
    fn on_message(&mut self, ctx: &mut Self::Context, msg: Self::Msg);
    fn on_stop(&mut self, ctx: &mut Self::Context) {
        println!("Actor exiting gracefully");
    }
}

impl<T> ActorRuntime for T where T: Actor + Sendable {}

pub trait ActorRuntime
where
    Self: Actor,
{
    fn spawn(mut self, args: Self::Arguments) -> Handle<Self::Msg> {
        let (signal_tx, mut signal_rx) = concurrency::oneshot();
        let (supervisor_tx, supervisor_rx) = concurrency::mpsc_bounded(32);
        let (message_tx, mut message_rx) = concurrency::mpsc_unbounded::<Self::Msg>();

        tokio::spawn(async move {
            let mut ctx = Self::pre_start(args).unwrap();

            loop {
                tokio::select! {
                    biased;
                    termination = &mut signal_rx => break,
                    Some(msg) = message_rx.recv() => {
                        self.on_message(&mut ctx, msg)
                    }
                }
            }
            self.on_stop(&mut ctx);
        });

        Handle {
            signal_tx: Some(signal_tx),
            supervisor_tx,
            message_tx,
        }
    }
}

pub struct Handle<T> {
    signal_tx: Option<concurrency::OneShotSender<Signal>>,
    supervisor_tx: concurrency::MpscSender<i32>, // make supervisor messages later
    message_tx: concurrency::UnboundedMpscSender<T>,
}

impl<M> Handle<M> {
    // send without waiting
    pub fn send(&mut self, msg: M)  {
        let _ = self.message_tx.send(msg);
    }
    pub fn kill(&mut self)  {
        let _ = self.signal_tx.take().unwrap().send(Signal::Kill);
    }
}

pub enum Signal {
    Kill,
}
