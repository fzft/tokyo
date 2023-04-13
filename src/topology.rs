use std::collections::HashSet;

use crate::actor::{Actor, ActorContext, Message};
use crate::engine::{IContext, IEngine};
use crate::types::{State, StateVal};

// stand alone tp is only one processor
pub trait StandAloneTopology
    where
        Self: IEngine
{
    fn spawn_stand_alone<M: Message>(&mut self, actor: impl Actor<M>, ctx: Self::Context) -> Result<(), String>;

    fn validate<M: Message>(&mut self, actor: impl Actor<M>, ctx: Self::Context) -> Result<(), String> {
        validate_state(ctx.state())?;
        match actor.ctx().downstream::<M>() {
            Some(_) => Err(format!("Unexpected downstream actor for current actor, expect StandAlone Topology")),
            None => Ok(())
        }
    }
}


// chain tp is link list tp
// tp will validate the tp
pub trait ChainTopology where Self: IEngine
{
    fn spawn_chain<M: Message>(&mut self, actor: impl Actor<M>) -> Result<(), String>;

    fn validate<M: Message>(&mut self, actor: impl Actor<M>, ctx: Self::Context) -> Result<(), String> {
        // validate_state(ctx.state())?;
        // let mut set = HashSet::new();
        unimplemented!()
    }
}


// DAG tp is more complex tp than chain
// tp will validate the tp, use strong connect to check whether cyclic
pub trait DAGTopology where Self: IEngine {
    fn spawn_dag<M: Message>(&mut self, actor: impl Actor<M>) -> Result<(), String> where M: Message;

    fn validate<M: Message>(&mut self, actor: impl Actor<M>, ctx: Self::Context) -> Result<(), String> where M: Message {
        unimplemented!()
    }
}

fn validate_state(s: State) -> Result<(), String> {
    match s {
        State::EngineState(s) => {
            match s {
                StateVal::Created => {}
                _ => return Err(format!("Unexpected engine state, topology validate should in created state"))
            }
        }
        _ => return Err(format!("Unexpected state category, expected engine state"))
    }

    Ok(())
}

