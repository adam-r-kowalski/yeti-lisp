extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;
use core::any::{Any, TypeId};
use tokio::sync::Mutex;

pub struct NativeType {
    pub value: Arc<Mutex<dyn Any + Send>>,
    pub type_id: TypeId,
    pub name: String,
}

impl NativeType {
    pub fn new<T: 'static + Send>(value: T, name: String) -> NativeType {
        NativeType {
            value: Arc::new(Mutex::new(value)),
            type_id: TypeId::of::<T>(),
            name,
        }
    }
}

impl PartialEq for NativeType {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl core::hash::Hash for NativeType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.value).hash(state);
    }
}

impl core::fmt::Debug for NativeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}({:?})", self.name, Arc::as_ptr(&self.value))
    }
}

impl core::fmt::Display for NativeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}({:?})", self.name, Arc::as_ptr(&self.value))
    }
}

impl Eq for NativeType {}

impl PartialOrd for NativeType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Arc::as_ptr(&self.value).partial_cmp(&Arc::as_ptr(&other.value))
    }
}

impl Ord for NativeType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Arc::as_ptr(&self.value).cmp(&Arc::as_ptr(&other.value))
    }
}

impl Clone for NativeType {
    fn clone(&self) -> Self {
        NativeType {
            value: self.value.clone(),
            type_id: self.type_id,
            name: self.name.clone(),
        }
    }
}
