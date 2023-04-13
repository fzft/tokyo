use tokio::sync::mpsc;
use uuid::Uuid;

use tokyo::actor::{Actor, Context, Handler};
use tokyo::engine::Engine;

struct Sum {
    a: usize,
    b: usize,
}

struct SumRet {
    sum: usize,
}

struct ActorA<Sum> {}

struct ActorB<SumRet> {}

impl<Sum> Actor<Sum> for ActorA<Sum> {
    type Context = Context<Sum>;
    type Id = CustomUuid;
}

impl<Sum> Handler<Sum> for ActorA<Sum> {
    type Result = SumRet;

    fn handle(&mut self, msg: Sum, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl<SumRet> Actor<SumRet> for ActorB<SumRet> {
    type Context = Context<SumRet>;
    type Id = CustomUuid;
}

impl<SumRet> Handler<SumRet> for ActorB<SumRet> {
    type Result = usize;

    fn handle(&mut self, msg: Self::Message, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let engine = Engine::new();
}
