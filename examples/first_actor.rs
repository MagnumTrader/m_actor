use m_actor::{Actor, ActorRuntime};
use std::time::Duration;
use tokio::sync::oneshot;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let x = Counter;

    let mut handle = x.spawn(());
    handle.send(CounterMsg::Increment);
    handle.send(CounterMsg::Increment);
    handle.send(CounterMsg::Increment);
    handle.send(CounterMsg::Increment);
    handle.send(CounterMsg::Increment);

    let (msg, rx) = CounterMsg::get();
    handle.send(msg);

    assert_eq!(5, rx.await.unwrap());

    std::thread::sleep(Duration::from_millis(500));
    handle.kill();

    std::thread::sleep(Duration::from_millis(500));
    Ok(())
}

enum CounterMsg {
    Increment,
    Get(oneshot::Sender<i32>),
}
impl CounterMsg {
    fn get() -> (CounterMsg, oneshot::Receiver<i32>) {
        let (tx, rx) = oneshot::channel();
        (CounterMsg::Get(tx), rx)
    }
}

struct Counter;

impl Actor for Counter {
    type Msg = CounterMsg;
    type Context = i32;
    type Arguments = ();

    fn pre_start(_arguments: Self::Arguments) -> m_actor::Result<Self::Context> {
        Ok(0)
    }

    fn on_message(&mut self, ctx: &mut Self::Context, msg: Self::Msg) {
        match msg {
            CounterMsg::Increment => *ctx += 1,
            CounterMsg::Get(sender) => sender.send(*ctx).unwrap(),
        }
    }

    fn on_stop(&mut self, _ctx: &mut Self::Context) {
        println!("I override the on_stop method, exiting now!")
    }
}
