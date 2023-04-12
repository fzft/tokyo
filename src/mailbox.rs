use crate::process::{Message, Processor};

// mailbox handle the inbound message to engine
pub trait Mailbox<M> where Self: Processor<M>, M: Message {

}