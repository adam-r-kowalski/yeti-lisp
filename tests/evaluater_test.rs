use im::{hashmap, vector, HashMap};
use rug::{Integer, Rational};
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
          |env, args| {
            let (env, args) = tao::evaluate_expressions(env, args);
            match &args[0] {
              tao::Expression::Integer(i) => (env, tao::Expression::Integer(i * Integer::from(2))),
              _ => panic!("Expected integer argument"),
            }
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
    let environment = tao::core::environment();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_if_then_branch() {
    let tokens = tao::tokenize("(if true 1 2)");
    let expression = tao::parse(tokens);
    let environment = tao::core::environment();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_if_else_branch() {
    let tokens = tao::tokenize("(if false 1 2)");
    let expression = tao::parse(tokens);
    let environment = tao::core::environment();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_def() {
    let tokens = tao::tokenize("(def x 5)");
    let expression = tao::parse(tokens);
    let environment = tao::core::environment();
    let (actual_environment, actual) = tao::evaluate(environment.clone(), expression);
    let expected = tao::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert("x".to_string(), tao::Expression::Integer(Integer::from(5)));
    assert_eq!(actual_environment, expected_environment);
}

#[test]
fn evaluate_array() {
    let tokens = tao::tokenize("[(+ 1 2) (/ 4 3)]");
    let expression = tao::parse(tokens);
    let environment = tao::core::environment();
    let (_, actual) = tao::evaluate(environment.clone(), expression);
    let expected = tao::Expression::Array(vector![
        tao::Expression::Integer(Integer::from(3)),
        tao::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_map() {
    let tokens = tao::tokenize("{:a (+ 1 2) :b (/ 4 3)}");
    let expression = tao::parse(tokens);
    let environment = tao::core::environment();
    let (_, actual) = tao::evaluate(environment.clone(), expression);
    let expected = tao::Expression::Map(hashmap! {
        tao::Expression::Keyword(":a".to_string()) => tao::Expression::Integer(Integer::from(3)),
        tao::Expression::Keyword(":b".to_string()) => tao::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    });
    assert_eq!(actual, expected);
}
