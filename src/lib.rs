#![no_std]
#![forbid(unsafe_code)]
#![feature(ip_in_core)]
#![feature(error_in_core)]
#![feature(iter_array_chunks)]
#![feature(async_closure)]
#![recursion_limit = "256"]

pub mod array;
pub mod atom;
pub mod core;
pub mod effect;
mod evaluator;
pub mod expression;
pub mod extract;
pub mod html;
pub mod http;
pub mod json;
pub mod map;
mod native_type;
mod numerics;
mod parser;
mod peeking_take_while;
pub mod server;
pub mod sql;
mod tokenizer;
pub mod toml;
pub mod yaml;

pub use evaluator::{evaluate, evaluate_expressions, evaluate_source, pattern_match};
pub use expression::{Environment, Expression};
pub use native_type::NativeType;
pub use numerics::{bits_to_decimal_digits, decimal_digits_to_bits, ratio, Float};
pub use parser::{parse, parse_module};
pub use peeking_take_while::PeekableExt;
pub use tokenizer::{Token, Tokens};
