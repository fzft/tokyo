// producer - event >   buffer - > consumer(worker)

use std::collections::HashMap;
use std::future::Future;
use std::ptr::null;
use std::sync::Arc;

use tokio::spawn;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

use crate::context::Context;
use crate::event_stream::EventStream;
use crate::opts::DefaultOpts;
use crate::pid::Pid;
use crate::process::Processor;

struct Registry<Message>
{
    lookup: HashMap<Uuid, Arc<Processor<Message>>>,
}

impl<Message> Registry<Message>
{
    fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    async fn remove(&mut self, u: Uuid) {
        self.lookup.remove(&u);
    }

    async fn add(&mut self, p: Arc<Processor<Message>>) {
        let p_clone = p.clone();
        let pid = p_clone.pid.lock().await;
        if self.lookup.contains_key(&pid.id) {
            return;
        }
        self.lookup.insert(pid.id, p);
    }

    async fn get(&mut self, u: Uuid) -> Option<Arc<Processor<Message>>> {
        self.lookup.get(&u).map(|processor| Arc::clone(processor))
    }
}


pub trait TReceiver
{
    fn receive<Message>(&self, ctx: &Context<Message>) where Self: Sized;
}


// Represents the actor engine
pub struct Engine<Message>
{
    pub event_stream: Arc<Mutex<EventStream>>,
    pub registry: Arc<Mutex<Registry<Message>>>,
}

impl<Message> Engine<Message>
{
    fn new() -> Self {
        Self {
            event_stream: Arc::new(Mutex::new(EventStream::new())),
            registry: Arc::new(Mutex::new(Registry::new())),
        }
    }

    async fn spawn(&self, p: Box<dyn TReceiver>, name: &str) -> Arc<Mutex<Pid>> {
        let mut opts = DefaultOpts::new(p);
        opts.name = String::from(name);
        let proc = Arc::new(Processor::new(Arc::new(Mutex::new(self.clone())), opts));
        self.spawn_proc(proc).await
    }

    async fn spawn_proc(&self, p: Arc<Processor<Message>>) -> Arc<Mutex<Pid>> {
        let mut registry = self.registry.lock().await;
        registry.add(p.clone()).await;
        p.clone().start().await;
        p.pid.clone()
    }
}

impl<Message> Clone for Engine<Message>
    where
        R: TReceiver + Send + Sync + 'static,
        P: Fn() -> R,
{
    fn clone(&self) -> Self {
        Self {
            event_stream: self.event_stream.clone(),
            registry: self.registry.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_event_stream() {
        struct A {}
        let mut e: Engine<A> = Engine::new();
        let mut subs = Vec::new();

        for _ in 0..10 {
            let mut stream = e.event_stream.lock().await;

        }
    }
}

