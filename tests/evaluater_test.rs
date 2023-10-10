use im::{hashmap, HashMap};
use rug::Integer;
use tao;

#[test]
fn evaluate_keyword() {
    let tokens = tao::tokenize(":x");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_string() {
    let tokens = tao::tokenize(r#""hello""#);
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_integer() {
    let tokens = tao::tokenize("5");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_float() {
    let tokens = tao::tokenize("3.14");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Float(tao::Float::from_str("3.14"));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_unbound_symbol() {
    let tokens = tao::tokenize("x");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Symbol("x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_symbol_bound_to_integer() {
    let tokens = tao::tokenize("x");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "x".to_string() => tao::Expression::Integer(Integer::from(5)),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_symbol_bound_to_function() {
    let tokens = tao::tokenize("(double 5)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "double".to_string() => tao::Expression::IntrinsicFunction(
          |arguments| match &arguments[0] {
            tao::Expression::Integer(i) => tao::Expression::Integer(i * Integer::from(2)),
            _ => panic!("Expected integer argument"),
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_add() {
    let tokens = tao::tokenize("(+ 5 3)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "+".to_string() => tao::Expression::IntrinsicFunction(
          |arguments| match (&arguments[0], &arguments[1]) {
            (tao::Expression::Integer(lhs), tao::Expression::Integer(rhs)) => tao::Expression::Integer((lhs + rhs).into()),
            _ => panic!("Expected integer argument"),
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
}
