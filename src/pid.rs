use uuid::Uuid;

use crate::process::ProcessCtx;
use crate::types::State;

pub trait Pid {
    fn id(&self) -> Uuid;
    fn state(&self) -> State;
    fn ctx(&self) -> dyn ProcessCtx;
}
