extern crate alloc;

use crate::effect::Effect;
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::Expression;
use im::Vector;

type Result<T> = core::result::Result<T, Effect>;

pub async fn get(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let array = &args[0];
    let key = &args[1];
    let map = extract::map(array.clone())?;
    if let Some(value) = map.get(key) {
        Ok((env, value.clone()))
    } else if args.len() == 3 {
        Ok((env, args[2].clone()))
    } else {
        Ok((env, Expression::Nil))
    }
}

pub async fn assoc(
    env: Environment,
    args: Vector<Expression>,
) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let (map, key, value) = (args[0].clone(), args[1].clone(), args[2].clone());
    let mut m = extract::map(map)?;
    m.insert(key, value);
    Ok((env, Expression::Map(m)))
}

pub async fn dissoc(
    env: Environment,
    args: Vector<Expression>,
) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let (map, key) = (args[0].clone(), args[1].clone());
    let mut m = extract::map(map)?;
    m.remove(&key);
    Ok((env, Expression::Map(m)))
}

pub async fn merge(
    env: Environment,
    args: Vector<Expression>,
) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let (map1, map2) = (args[0].clone(), args[1].clone());
    let mut map1 = extract::map(map1)?;
    let map2 = extract::map(map2)?;
    map1.extend(map2);
    Ok((env, Expression::Map(map1)))
}
