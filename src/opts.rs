use std::sync::Arc;

use crate::engine::TReceiver;

pub struct DefaultOpts
{
    pub p: Arc<Box<dyn TReceiver>> ,
    pub name: String,
}

impl DefaultOpts

{
    pub fn new(p: Box<dyn TReceiver>) -> Self {
        Self {
            p: Arc::new(p),
            name: String::new(),
        }
    }
}