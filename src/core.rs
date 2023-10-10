use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction};
use im::{hashmap, HashMap};

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (Integer(lhs), Integer(rhs)) => Integer((lhs + rhs).into()),
            _ => panic!("Expected integer argument"),
          }
        ),
    }
}
