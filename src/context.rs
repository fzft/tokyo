use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::engine::{Engine, TReceiver};
use crate::pid::Pid;

pub struct Context<Message>
{
    pub pid: Arc<Mutex<Pid>>,
    pub sender: Option<Arc<Mutex<Pid>>>,
    pub engine: Arc<Mutex<Engine<Message>>>,
    pub receiver: Option<Box<dyn TReceiver>>,
    pub parentCtx: Option<Arc<Mutex<Context<Message>>>>,
    pub children: Mutex<Arc<HashMap<Uuid, Pid>>>,
    pub message: Option<Arc<Mutex<Message>>>,
}

impl<Message> Context<Message>
{
    pub(crate) fn new(engine: Arc<Mutex<Engine<Message>>>, pid: Arc<Mutex<Pid>>) -> Self {
        Self {
            engine,
            pid,
            sender: None,
            receiver: None,
            children: Mutex::new(Arc::new(HashMap::new())),
            parentCtx: None,
            message: None,
        }
    }
}