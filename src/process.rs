use std::sync::Arc;

use tokio::sync::Mutex;

use crate::context::Context;
use crate::engine::{Engine, TReceiver};
use crate::opts::DefaultOpts;
use crate::pid::{Pid, State};
use crate::types::{AppMessage, DefaultMessage};

pub struct Envelope<Message>{
    pub msg: AppMessage<Message>,
    pub sender: Arc<Mutex<Pid>>,
}

pub struct Processor<Message>
{
    pub pid: Arc<Mutex<Pid>>,
    pub ctx: Arc<Mutex<Context<Message>>>,
    pub opts: DefaultOpts,
    pub m_buffer: Option<Arc<Mutex<Vec<Envelope<Message>>>>>,
}

impl<Message> Processor<Message>
{
    pub fn new(engine: Arc<Mutex<Engine<Message>>>, opts: DefaultOpts) -> Self {
        let pid = Arc::new(Mutex::new(
            Pid::new(opts.name.as_str())
        ));
        Self {
            pid,
            ctx: Arc::new(Mutex::new(Context::new(engine, pid.clone()))),
            opts,
            m_buffer: None,
        }
    }

    pub async fn start(self: Arc<Self>) {
        let recv_maker = Arc::clone(&self.opts.p);
        let recv = recv_maker();

        let mut ctx = self.ctx.lock().await;
        ctx.receiver = Some(recv);

        let mut pid = self.pid.lock().await;
        pid.state = State::Initialized;

        pid.state = State::Started;
        let engine = ctx.engine.clone();
        let mut e = engine.lock().await;
        let mut event_stream = e.event_stream.lock().await;
        event_stream.publish(AppMessage::Default(DefaultMessage::ActivationEvent(self.pid.clone()))).await;

        if let Some(m_buffer) = &self.m_buffer {
            let mut buf = m_buffer.lock().await;
            if !buf.is_empty() {
                self.invoke(buf.clone()).await;
                buf.clear();
            }
        }
    }

    async fn invoke(self: Arc<Self <>>, msgs: Vec<Envelope<Message>>) {
        let mut nproc = 0;
        for msg in msgs {
            nproc += 1;
            let mut ctx = self.ctx.lock().await;
            let mut message = ctx.message.lock().await;
            message = msg.msg;
            ctx.sender = Some(self.pid.clone());
            if let Some(r) = &ctx.receiver {
                r.receive(&*ctx)
            }
        }
    }

    async fn cleanup() {}
}

