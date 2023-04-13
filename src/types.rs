#[derive(Clone, Copy, PartialEq)]
pub enum State {
    EngineState(StateVal),
    ResultState(StateVal),
    ActorState(StateVal)
}

#[derive(Copy, Clone, PartialEq)]
pub enum StateVal {
    Created,
    Initialized,
    Started,
    Running,
    Stopping,
    Stopped,
    Fail,
    Success,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TopologyVal {
    StandAlone,
    Chain,
    DAG,
}

