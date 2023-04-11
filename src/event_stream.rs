use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::sync::Arc;
use uuid::Uuid;
use crate::types::AppMessage;

type F<Message> = Fn(&AppMessage<Message>) -> dyn(Send + Sync) ;

pub struct EventStream<Message>
{
    subs: HashMap<Uuid, Arc<Box<dyn Fn()>>>
}


impl<Message> EventStream<Message>
{
    pub fn new() -> Self {
        Self {
            subs: HashMap::new()
        }
    }

    pub async fn unsubscribe(&mut self, u: Uuid) {
        self.subs.remove(&u);
    }

    pub async fn subscribe<Fut, Message, F>(&mut self, f: F) -> Uuid
        where
            F: Fn(&AppMessage<Message>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
    {
        let u = Uuid::new_v4();
        self.subs.insert(u, Arc::new(f));
        u
    }

    pub async fn publish<Fut, Message, F>(&mut self, m: AppMessage<Message>)
        where
            F: Fn(&AppMessage<Message>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
    {
        for f in self.subs.values() {
            let fut = (*f)(&m);
            tokio::spawn(async move {
                fut.await;
            });
        }
    }
}
