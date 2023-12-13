#![no_std]
#![forbid(unsafe_code)]
#![feature(error_in_core)]

pub mod array;
pub mod atom;
pub mod channel;
pub mod effect;
mod evaluator;
pub mod expression;
pub mod extract;
pub mod map;
mod native_type;
mod numerics;
mod parser;
mod tokenizer;

pub use evaluator::{evaluate, evaluate_expressions, evaluate_source, pattern_match};
pub use expression::{Environment, Expression};
pub use native_type::NativeType;
pub use numerics::{bits_to_decimal_digits, decimal_digits_to_bits, ratio, Float};
pub use parser::{parse, parse_all};
pub use tokenizer::{tokenize, Token};
