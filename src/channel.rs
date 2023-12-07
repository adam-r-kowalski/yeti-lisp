extern crate alloc;

use crate::Expression;
use crate::effect::Effect;
use crate::effect::error;
use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub struct Channel {
    pub sender: mpsc::Sender<Expression>,
    pub receiver: Arc<Mutex<mpsc::Receiver<Expression>>>,
    pub closed: Arc<AtomicBool>,
}

impl Channel {
    pub fn new(buffer_size: usize) -> Channel {
        let (sender, receiver) = mpsc::channel(buffer_size);
        let receiver = Arc::new(Mutex::new(receiver));
        let closed = Arc::new(AtomicBool::new(false));
        Channel {
            sender,
            receiver,
            closed,
        }
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
        write!(f, "#channel({:?})", Arc::as_ptr(&self.receiver))
    }
}

impl core::fmt::Display for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#channel({:?})", Arc::as_ptr(&self.receiver))
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
            closed: self.closed.clone(),
        }
    }
}

pub async fn put(chan: Channel, value: Expression) -> Result<(), Effect> {
    if value == Expression::Nil {
        chan.closed.store(true, core::sync::atomic::Ordering::Relaxed);
        return Ok(());
    }
    if chan.closed.load(core::sync::atomic::Ordering::Relaxed) {
        return Ok(());
    }
    chan.sender.send(value).await.map_err(|_| error("Channel closed"))?;
    Ok(())
}

pub async fn take(chan: Channel) -> Result<Expression, Effect> {
    if chan.closed.load(core::sync::atomic::Ordering::Relaxed) {
        if let Ok(value) = chan.receiver.lock().await.try_recv() {
            return Ok(value);
        }
        return Ok(Expression::Nil);
    }
    let value = chan.receiver.lock().await.recv().await.ok_or(error("Channel closed"))?;
    Ok(value)
}
