#![no_std]
#![forbid(unsafe_code)]
#![feature(ip_in_core)]

pub mod core;
pub mod effect;
mod evaluator;
mod expression;
pub mod extract;
mod numerics;
mod parser;
mod peeking_take_while;
mod tokenizer;

pub use evaluator::{evaluate, evaluate_expressions};
pub use expression::{Environment, Expression};
pub use numerics::{bits_to_decimal_digits, decimal_digits_to_bits, Float};
pub use parser::parse;
pub use peeking_take_while::PeekableExt;
pub use tokenizer::{Token, Tokens};
