extern crate alloc;
use crate::atom::Atom;
use crate::effect::{error, Effect};
use crate::expression::Call;
use crate::{Environment, Expression, NativeType};
use alloc::format;
use alloc::string::{String, ToString};
use im::{OrdMap, Vector};

type Result<T> = core::result::Result<T, Effect>;

pub fn map(expr: Expression) -> Result<OrdMap<Expression, Expression>> {
    match expr {
        Expression::Map(m) => Ok(m),
        _ => Err(error("Expected map")),
    }
}

pub fn key(map: OrdMap<Expression, Expression>, key: &str) -> Result<Expression> {
    match map.get(&Expression::Keyword(key.to_string())) {
        Some(expr) => Ok(expr.clone()),
        None => Err(error(&format!("Expected keyword {}", key))),
    }
}

pub fn string(expr: Expression) -> Result<String> {
    match expr {
        Expression::String(s) => Ok(s),
        _ => Err(error("Expected string")),
    }
}

pub fn keyword(expr: Expression) -> Result<String> {
    match expr {
        Expression::Keyword(k) => Ok(k),
        _ => Err(error("Expected keyword")),
    }
}

pub fn symbol(expr: Expression) -> Result<String> {
    match expr {
        Expression::Symbol(s) => Ok(s),
        _ => Err(error("Expected keyword")),
    }
}

pub fn array(expr: Expression) -> Result<Vector<Expression>> {
    match expr {
        Expression::Array(a) => Ok(a),
        _ => Err(error("Expected array")),
    }
}

pub fn call(expr: Expression) -> Result<Call> {
    match expr {
        Expression::Call(call) => Ok(call),
        _ => Err(error("Expected call")),
    }
}

pub fn integer(expr: Expression) -> Result<rug::Integer> {
    match expr {
        Expression::Integer(i) => Ok(i),
        _ => Err(error("Expected integer")),
    }
}

pub fn native_type(expr: Expression) -> Result<NativeType> {
    match expr {
        Expression::NativeType(t) => Ok(t),
        _ => Err(error("Expected native type")),
    }
}

pub fn module(expr: Expression) -> Result<Environment> {
    match expr {
        Expression::Module(m) => Ok(m),
        _ => Err(error("Expected module")),
    }
}

pub fn atom(expr: Expression) -> Result<Atom> {
    match expr {
        Expression::Atom(a) => Ok(a),
        _ => Err(error("Expected module")),
    }
}
