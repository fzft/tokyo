use uuid::Uuid;

pub enum State {
    Ready,
    Initialized,
    Started,
    Running,
    Stopped,
    Success,
    Fail,
}

pub struct Pid {
    pub name: String,
    pub id: Uuid,
    pub state: State,
}

impl Pid {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            id: Uuid::new_v4(),
            state: State::Ready,
        }
    }
}
