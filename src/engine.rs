// producer - event >   buffer - > consumer(worker)

use std::any::Any;
use std::collections::HashMap;

use uuid::Uuid;

use crate::actor::{Actor, ActorContext, Message};
use crate::topology::StandAloneTopology;
use crate::types::{State, StateVal, TopologyVal};

pub trait IEngine {
    type Context: IContext;

    fn start(&mut self);
}

pub trait IContext {
    fn stop(&mut self);

    fn terminate(&mut self);

    fn state(&self) -> State;
}

pub struct Engine {
    ctx: EngineCtx,
}


impl Engine {
    pub fn new() -> Self {
        Self {
            ctx: EngineCtx::new()
        }
    }
}

impl IEngine for Engine {
    type Context = EngineCtx;

    fn start(&mut self) {
        todo!()
    }
}


impl StandAloneTopology for Engine {
    fn spawn_stand_alone<M: Message>(&mut self, actor: impl Actor<M>, ctx: Self::Context) -> Result<(), String> {
        match self.validate(actor, ctx) {
            Ok(_) => {
                self.ctx.tp = Some(TopologyVal::StandAlone);
                self.ctx.state = State::EngineState(StateVal::Initialized);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}


pub struct EngineCtx
{
    tp: Option<TopologyVal>,
    state: State,
    actors: HashMap<Uuid, Box<dyn Any>>,
}

impl EngineCtx {
    fn new() -> Self {
        Self {
            tp: None,
            state: State::EngineState(StateVal::Created),
            actors: HashMap::new(),
        }
    }

    pub fn insert_actor<M>(&mut self, id: Uuid, ctx: impl ActorContext<M>) where M: Message  {
        self.actors.insert(id, Box::new(ctx) as Box<dyn Any>);
    }
}


impl IContext for EngineCtx {
    fn stop(&mut self) {
        todo!()
    }

    fn terminate(&mut self) {
        todo!()
    }

    fn state(&self) -> State {
        self.state
    }
}






