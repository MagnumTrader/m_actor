//! this is the main document comments
//!
//!
//!
//!
//!

use mactor::{Actor, Handle, Reciever};

use std::time::Duration;
use tokio::task::JoinHandle;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {

    let generator = Generator;

    let mut gen_handle = generator.spawn(());
    

    // im going to comment like this
    let mut current = 0;
    // and this bru

    while let Some(msg) = gen_handle.recv().await {
        match msg {
            GenMessage::Add => current += 1
        };
        println!("{current}");
        if current == 3 {
            gen_handle.shutdown();
        }
    };

    println!("main loop exited because handle returned None!");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

struct Generator;
struct GenHandle {
    rx: tokio::sync::mpsc::Receiver<GenMessage>,
    handle: JoinHandle<()>,
}

enum GenMessage {
    Add
}

impl mactor::Handle for GenHandle {
    fn shutdown(&mut self) {
        self.handle.abort();
    }

    async fn is_alive(&mut self) -> bool {
        !self.handle.is_finished()
    }
}

impl mactor::Reciever<GenMessage> for GenHandle {
    async fn recv(&mut self) -> Option<GenMessage> {
        self.rx.recv().await
    }
}

impl Actor for Generator {

    type Handle = GenHandle;

    /// Multiplier for the generator
    type Context = ();

    #[allow(unreachable_code)] // you break the loop by cannling shutdown on the handle
    fn spawn(self, _context: Self::Context) -> Self::Handle {
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        let handle = tokio::spawn(async move {
            println!("task is starting");

            loop {
                let _ = tx.send(GenMessage::Add).await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            };
            println!("task is now ending");
        });

        GenHandle { rx, handle }
    }
}
