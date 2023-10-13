use crate::evaluate_expressions;
use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction, Ratio};
use im::{hashmap, HashMap};
use rug;

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs + rhs).into()))),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "-".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs - rhs).into()))),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "*".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs * rhs).into()))),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "/".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => {
                let rational = rug::Rational::from((lhs, rhs));
                if rational.is_integer() {
                    Ok((env, Integer(rational.numer().clone())))
                } else {
                    Ok((env, Ratio(rational)))
                }
              },
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "if".to_string() => IntrinsicFunction(
          |env, args| {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = crate::evaluate(env, condition)?;
            match condition {
                Expression::Nil => crate::evaluate(env, otherwise),
                Expression::Bool(false) => crate::evaluate(env, otherwise),
                _ => crate::evaluate(env, then),
            }
          }
        ),
        "def".to_string() => IntrinsicFunction(
          |env, args| {
            let (name, value) = (args[0].clone(), args[1].clone());
            let (env, value) = crate::evaluate(env, value)?;
            let name = match name {
                Expression::Symbol(s) => s,
                _ => panic!("Expected symbol"),
            };
            let mut new_env = env.clone();
            new_env.insert(name, value);
            Ok((new_env, Expression::Nil))
          }
        ),
        "assoc".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map, key, value) = (args[0].clone(), args[1].clone(), args[2].clone());
            match map {
                Expression::Map(m) => {
                    let mut new_map = m.clone();
                    new_map.insert(key, value);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => panic!("Expected map"),
            }
          }
        ),
        "dissoc".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map, key) = (args[0].clone(), args[1].clone());
            match map {
                Expression::Map(m) => {
                    let mut new_map = m.clone();
                    new_map.remove(&key);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => panic!("Expected map"),
            }
          }
        ),
        "merge".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map1, map2) = (args[0].clone(), args[1].clone());
            match (map1, map2) {
                (Expression::Map(m1), Expression::Map(m2)) => {
                    let mut new_map = m1.clone();
                    new_map.extend(m2);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => panic!("Expected map"),
            }
          }
        ),
    }
}
