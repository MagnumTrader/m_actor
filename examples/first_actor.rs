#![allow(unused, unreachable_code)]

use std::time::Duration;

use mactor::{Actor, ActorRuntime, Handle};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let x = Hello;
    let y = 5;

    let mut handle = x.spawn(Box::new(move || 10 + y));

    handle.send(1);
    handle.send(1);
    handle.send(1);
    handle.send(1);
    handle.send(1);

    std::thread::sleep(Duration::from_millis(500));
    handle.kill();

    std::thread::sleep(Duration::from_millis(500));
    Ok(())
}

struct Hello;
impl Actor for Hello {
    type Msg = i32;
    type Context = i32;
    type Arguments = Box<dyn Fn() -> i32 + Send>; // can be a function that returns something else

    fn pre_start(arguments: Self::Arguments) -> mactor::Result<Self::Context> {
        Ok(arguments())
    }

    fn on_message(&mut self, ctx: &mut Self::Context, msg: Self::Msg) {
        *ctx += msg;
        println!("{}", ctx)
    }
    fn on_stop(&mut self, ctx: &mut Self::Context) {
        println!("omg i override this stuff")
    }
}
