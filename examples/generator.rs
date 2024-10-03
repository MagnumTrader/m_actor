#![allow(unused, unreachable_code)]


use std::{marker::PhantomData, time::Duration};

use mactor::{Actor, Handle, Reciever};
use tokio::task::JoinHandle;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[tokio::main]

async fn main() -> Result<()> {

    let supeer = SuperVisor::<AmazingStrategy>::new();
    supeer.doit(AmazingStrategy);

    let generator = Generator;

    let mut gen_handle = generator.spawn(());
    

    let mut current = 0;
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

    fn spawn(self, context: Self::Context) -> Self::Handle {
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        let handle = tokio::spawn(async move {
            println!("task is starting");
            loop {
                tx.send(GenMessage::Add).await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            println!("task is now ending");
        });

        GenHandle { rx, handle }
    }
}

#[derive(Debug)]
struct AmazingStrategy;

struct SuperVisor<T>{
    p: PhantomData<T>
}

impl Actor for SuperVisor<AmazingStrategy> {
    type Handle = GenHandle;

    type Context = ();

    fn spawn(self, context: Self::Context) -> Self::Handle {
        todo!()
    }
}


impl <T: std::fmt::Debug> SuperVisor<T> {
    fn new() -> Self {
        Self {
            p: PhantomData,
        }
    }
    fn doit(&self, t: T)  {
        println!("it works bru {:?}", t);
    }
}

