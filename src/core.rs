use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction, Ratio};
use im::{hashmap, HashMap};
use rug::Rational;

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (Integer(lhs), Integer(rhs)) => Integer((lhs + rhs).into()),
            _ => panic!("Expected integer argument"),
          }
        ),
        "-".to_string() => IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (Integer(lhs), Integer(rhs)) => Integer((lhs - rhs).into()),
            _ => panic!("Expected integer argument"),
          }
        ),
        "*".to_string() => IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (Integer(lhs), Integer(rhs)) => Integer((lhs * rhs).into()),
            _ => panic!("Expected integer argument"),
          }
        ),
        "/".to_string() => IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (Integer(lhs), Integer(rhs)) => Ratio(Rational::from((lhs, rhs))),
            _ => panic!("Expected integer argument"),
          }
        ),
    }
}
