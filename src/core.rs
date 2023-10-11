use crate::evaluate_arguments;
use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction, Ratio};
use im::{hashmap, HashMap};
use rug;

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => (env, Integer((lhs + rhs).into())),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "-".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => (env, Integer((lhs - rhs).into())),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "*".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => (env, Integer((lhs * rhs).into())),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "/".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => {
                let rational = rug::Rational::from((lhs, rhs));
                if rational.is_integer() {
                    (env, Integer(rational.numer().clone()))
                } else {
                    (env, Ratio(rational))
                }
              },
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "if".to_string() => IntrinsicFunction(
          |env, args| {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = crate::evaluate(env, condition);
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
            let (env, value) = crate::evaluate(env, value);
            let name = match name {
                Expression::Symbol(s) => s,
                _ => panic!("Expected symbol"),
            };
            let mut new_env = env.clone();
            new_env.insert(name, value);
            (new_env, Expression::Nil)
          }
        ),
    }
}
