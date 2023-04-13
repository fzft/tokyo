use crate::actor::Message;
use crate::ringbuffer::LockFreeRingBuffer;

const BUFFER_SIZE: usize = 1024;


// mailbox handle the inbound message to engine
pub struct Mailbox<M> where M: Message {
    rb: LockFreeRingBuffer<M>,
}

impl<M> Mailbox<M>
    where M: Message
{
    pub fn new() -> Self {
        Self {
            rb: LockFreeRingBuffer::new(BUFFER_SIZE)
        }
    }
}