use crate::actor::{Message, Actor};

// mailbox handle the inbound message to engine
pub trait Mailbox<M> where Self: Actor<M>, M: Message {

}