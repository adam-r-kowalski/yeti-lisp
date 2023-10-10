use crate::Expression;
use crate::Expression::Integer;
use im::{hashmap, HashMap};
use rug;

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "x".to_string() => Integer(rug::Integer::from(5)),
        "y".to_string() => Integer(rug::Integer::from(10)),
    }
}
