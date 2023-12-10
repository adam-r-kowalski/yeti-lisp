extern crate alloc;

use crate::Expression;
use alloc::sync::Arc;
use tokio::sync::Mutex;

pub struct Atom(pub Arc<Mutex<Expression>>);

impl Atom {
    pub fn new(expression: Expression) -> Atom {
        Atom(Arc::new(Mutex::new(expression)))
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl core::hash::Hash for Atom {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}

impl core::fmt::Debug for Atom {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#atom({:?})", Arc::as_ptr(&self.0))
    }
}

impl core::fmt::Display for Atom {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#atom({:?})", Arc::as_ptr(&self.0))
    }
}

impl Eq for Atom {}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Arc::as_ptr(&self.0).cmp(&Arc::as_ptr(&other.0))
    }
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        Atom(Arc::clone(&self.0))
    }
}
