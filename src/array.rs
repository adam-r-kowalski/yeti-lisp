extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::Expression;
use im::Vector;

type Result<T> = core::result::Result<T, Effect>;

pub fn nth(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let arr = extract::array(args[0].clone())?;
    let idx = extract::integer(args[1].clone())?
        .to_usize()
        .ok_or_else(|| error("Index out of range"))?;
    if let Some(value) = arr.get(idx) {
        Ok((env, value.clone()))
    } else if args.len() == 3 {
        Ok((env, args[2].clone()))
    } else {
        Err(error("Index out of range"))
    }
}

pub fn count(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let arr = extract::array(args[0].clone())?;
    Ok((env, Expression::Integer(arr.len().into())))
}
