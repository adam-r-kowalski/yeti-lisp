extern crate alloc;

use crate::Expression;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use im::{vector, Vector};

#[derive(PartialEq, Eq, Clone)]
pub struct Effect {
    pub kind: String,
    pub arguments: Vector<Expression>,
}

pub fn error(message: &str) -> Effect {
    Effect {
        kind: "error".to_string(),
        arguments: vector![Expression::String(message.to_string())],
    }
}

impl core::fmt::Debug for Effect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "#effect({} {})",
            self.kind,
            self.arguments
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
