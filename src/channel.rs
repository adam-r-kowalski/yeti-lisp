extern crate alloc;

use alloc::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use crate::Expression;

pub struct Channel {
    pub sender: mpsc::Sender<Expression>,
    pub receiver: Arc<Mutex<mpsc::Receiver<Expression>>>,
}

impl Channel {
    pub fn new() -> Channel {
        let (sender, receiver) = mpsc::channel(1);
        let receiver = Arc::new(Mutex::new(receiver));
        Channel { sender, receiver }
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.receiver, &other.receiver)
    }
}

impl core::hash::Hash for Channel {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.receiver).hash(state);
    }
}

impl core::fmt::Debug for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#atom({:?})", Arc::as_ptr(&self.receiver))
    }
}

impl core::fmt::Display for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#atom({:?})", Arc::as_ptr(&self.receiver))
    }
}

impl Eq for Channel {}

impl PartialOrd for Channel {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Arc::as_ptr(&self.receiver).partial_cmp(&Arc::as_ptr(&other.receiver))
    }
}

impl Ord for Channel {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Arc::as_ptr(&self.receiver).cmp(&Arc::as_ptr(&other.receiver))
    }
}

impl Clone for Channel {
    fn clone(&self) -> Self {
        Channel {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}
