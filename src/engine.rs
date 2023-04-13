// producer - event >   buffer - > consumer(worker)

use crate::types::State;

pub trait Engine: 'static {
    type Context: EngineCtx;

    fn start(&mut self);

    // spawn will spawn processor with certain topology
    // if spawn success return the processor id
    // otherwise return error
}


pub trait EngineCtx {
    fn state(&self) -> State;
}






