extern crate alloc;

use alloc::string::{String, ToString};
use core::error::Error;

#[derive(PartialEq, Eq, Clone)]
pub enum Effect {
    Error(String),
}

pub fn error(message: &str) -> Effect {
    Effect::Error(message.to_string())
}

impl core::fmt::Debug for Effect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Effect::Error(message) => write!(f, "#effect::error({})", message),
        }
    }
}

impl core::fmt::Display for Effect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Effect::Error(message) => write!(f, "#effect::error({})", message),
        }
    }
}

impl Error for Effect {}
