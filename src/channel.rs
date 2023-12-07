extern crate alloc;

use crate::Expression;
use async_channel::{bounded, Receiver, Sender};
use uuid::Uuid;

#[derive(Clone)]
pub struct Channel {
    pub sender: Sender<Expression>,
    pub receiver: Receiver<Expression>,
    pub uuid: Uuid,
}

impl Channel {
    pub fn new(buffer_size: usize) -> Channel {
        let (sender, receiver) = bounded(buffer_size);
        let uuid = Uuid::new_v4();
        Channel {
            sender,
            receiver,
            uuid,
        }
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl core::hash::Hash for Channel {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl core::fmt::Debug for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#channel({:?})", self.uuid)
    }
}

impl core::fmt::Display for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#channel({:?})", self.uuid)
    }
}

impl Eq for Channel {}

impl PartialOrd for Channel {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

impl Ord for Channel {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

pub async fn take(channel: Channel) -> Expression {
    channel.receiver.recv().await.unwrap_or(Expression::Nil)
}
