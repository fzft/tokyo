use std::sync::Arc;

use tokio::sync::Mutex;

use crate::pid::Pid;

pub enum AppMessage<CustomMessage> {
    Default(DefaultMessage),
    Custom(CustomMessage),
}


pub enum DefaultMessage {
    ActivationEvent(Arc<Mutex<Pid>>),
}