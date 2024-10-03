#![allow(unused, unreachable_code)]

use std::time::Duration;

use mactor::{Actor, Sender};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let counter = Counter(0);
    let mut handle = counter.spawn(CounterContext::new());

    handle.send(CounterMessage::Increment(2)).await;
    handle.send(CounterMessage::Increment(2)).await;
    handle.send(CounterMessage::Increment(2)).await;
    handle.send(CounterMessage::Increment(10)).await;
    handle.send(CounterMessage::Decrement(6)).await;
    handle.send(CounterMessage::Reset).await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

struct Counter(usize);

#[derive(Debug)]
pub enum CounterMessage {
    Increment(usize),
    Decrement(usize),
    Reset,
}

impl Actor for Counter {
    type Context = CounterContext;
    type Handle = CounterHandle;

    fn spawn(mut self, mut context: Self::Context) -> Self::Handle {
        let (tx, mut rx) = tokio::sync::mpsc::channel(512);

        tokio::spawn(async move {
            println!("Spawning Actor!");
            println!("Initial value {}", self.0);

            while let Some(msg) = rx.recv().await {
                context.messages += 1;
                let current = &mut self.0;
                println!("recieved action: {:?}", msg);
                match msg {
                    CounterMessage::Increment(x) => *current = current.saturating_add(x),
                    CounterMessage::Decrement(x) => *current = current.saturating_sub(x),
                    CounterMessage::Reset => *current = 0,
                }
                println!("current value {}", self.0);
                println!("totalt messages recieved {}", context.messages);
            }
        });
        CounterHandle { tx }
    }
}

struct CounterHandle {
    tx: tokio::sync::mpsc::Sender<CounterMessage>,
}

impl mactor::Handle for CounterHandle {
    fn shutdown(&mut self) {
        todo!()
    }
    async fn is_alive(&mut self) -> bool {
        todo!()
    }
}

impl mactor::Sender<CounterMessage> for CounterHandle {
    async fn send(&mut self, message: CounterMessage) {
        self.tx.send(message).await;
    }
}

struct CounterContext {
    messages: usize,
}
impl CounterContext {
    fn new() -> Self {
        CounterContext {
            messages: Default::default(),
        }
    }
}
