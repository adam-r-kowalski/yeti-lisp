use crate::evaluate_arguments;
use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction, Ratio};
use im::{hashmap, HashMap};
use rug;

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |env, args| {
            let (_, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Integer((lhs + rhs).into()),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "-".to_string() => IntrinsicFunction(
          |env, args| {
            let (_, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Integer((lhs - rhs).into()),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "*".to_string() => IntrinsicFunction(
          |env, args| {
            let (_, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Integer((lhs * rhs).into()),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
        "/".to_string() => IntrinsicFunction(
          |env, args| {
            let (_, args) = evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => {
                let rational = rug::Rational::from((lhs, rhs));
                if rational.is_integer() {
                    Integer(rational.numer().clone())
                } else {
                    Ratio(rational)
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
            let (_, e) = match condition {
                Expression::Nil => crate::evaluate(env, otherwise),
                Expression::Bool(false) => crate::evaluate(env, otherwise),
                _ => crate::evaluate(env, then),
            };
            e
          }
        ),
    }
}
