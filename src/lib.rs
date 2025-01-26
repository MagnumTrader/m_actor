//! # m_actor - Market Actor Framework
//! A lightweight actor framework for concurrent message processing in Rust.
//!
//! ## Overview
//!
//! This module provides a trait-based actor system with:
//! - Spawn and message handling capabilities
//! - Concurrent message passing
//! - Flexible actor lifecycle management
//!
//! ## Key Components
//!
//! - [`Actor`]: Defines the core behavior for actors
//! - [`Handle`]: Provides interaction with spawned actors
//! - [`ActorCell`]: Internal management of actor communication channels
pub mod concurrency;
use concurrency::{
    MpscReciever, MpscSender, OneShotReciever, OneShotSender, UnboundedMpscReciever,
    UnboundedMpscSender,
};

mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub trait Sendable: Sized + Send + Sync + 'static {}
impl<T> Sendable for T where T: Sized + Send + Sync + 'static {}

pub trait Actor: Sized + Send + 'static {
    type Msg: Send;
    type Context: Send;
    type Arguments: Send;

    /// Implement any startup logic needed for creating the context.
    /// Will get called first after calling Actor.spawn().
    fn pre_start(arguments: Self::Arguments) -> Result<Self::Context>;

    /// Messaging logic, all actors must handle their messages
    fn on_message(&mut self, ctx: &mut Self::Context, msg: Self::Msg);

    /// Optional implementation.
    /// Do shutdown logic for cleanup etc
    #[allow(unused_variables)]
    fn on_stop(&mut self, ctx: &mut Self::Context) {
        println!("Actor exiting gracefully");
    }
}

impl<T> ActorRuntime for T where T: Actor {}

pub trait ActorRuntime
where
    Self: Actor,
{
    fn spawn(mut self, args: Self::Arguments) -> Handle<Self::Msg> {
        let (cell, mut signal_rx, _, mut message_rx) = ActorCell::new();

        tokio::spawn(async move {
            let mut ctx = Self::pre_start(args).unwrap();
            loop {
                tokio::select! {
                    biased;
                    _ = &mut signal_rx => break,
                    Some(msg) = message_rx.recv() => {
                        self.on_message(&mut ctx, msg)
                    }
                }
            }
            self.on_stop(&mut ctx);
        });
        Handle::new(cell)
    }
}

pub struct Handle<M> {
    actor_cell: ActorCell<M>,
}

impl<M> Handle<M> {
    pub(crate) fn new(actor_cell: ActorCell<M>) -> Handle<M> {
        Handle { actor_cell }
    }
}

pub(crate) struct ActorCell<M> {
    // we can add here all the details later
    signal_tx: Option<OneShotSender<Signal>>,
    #[allow(unused)] // will implement this later
    supervisor_tx: Option<MpscSender<i32>>, // make supervisor messages later
    message_tx: UnboundedMpscSender<M>,
}

impl<M> Drop for ActorCell<M> {
    fn drop(&mut self) {
        if let Some(tx) = self.signal_tx.take() {
            let _ = tx.send(Signal);
        }
    }
}

impl<M> ActorCell<M> {
    pub(crate) fn new() -> (
        ActorCell<M>,
        OneShotReciever<Signal>,
        Option<MpscReciever<i32>>,
        UnboundedMpscReciever<M>,
    ) {
        let (signal_tx, signal_rx) = concurrency::oneshot();
        let (message_tx, message_rx) = concurrency::mpsc_unbounded::<M>();

        (
            ActorCell {
                signal_tx: Some(signal_tx),
                supervisor_tx: None,
                message_tx,
            },
            signal_rx,
            None,
            message_rx,
        )
    }
}

impl<M> Handle<M> {
    pub fn send(&mut self, msg: M) {
        let _ = self.actor_cell.message_tx.send(msg);
    }

    pub fn kill(&mut self) {
        let _ = self.actor_cell.signal_tx.take().unwrap().send(Signal);
    }
}

pub(crate) struct Signal;
