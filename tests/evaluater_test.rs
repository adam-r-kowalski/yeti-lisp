use std::sync::Arc;

use yeti;
use im::{hashmap, vector, HashMap};
use rug::{Integer, Rational};
use spin::Mutex;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn evaluate_keyword() -> Result {
    let tokens = yeti::Tokens::from_str(":x");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_string() -> Result {
    let tokens = yeti::Tokens::from_str(r#""hello""#);
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_integer() -> Result {
    let tokens = yeti::Tokens::from_str("5");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_float() -> Result {
    let tokens = yeti::Tokens::from_str("3.14");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Float(yeti::Float::from_str("3.14"));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_symbol_bound_to_integer() -> Result {
    let tokens = yeti::Tokens::from_str("x");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment {
        bindings: hashmap! {
            "x".to_string() => yeti::Expression::Integer(Integer::from(5)),
        },
        servers: Arc::new(Mutex::new(HashMap::new())),
    };
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_symbol_bound_to_function() -> Result {
    let tokens = yeti::Tokens::from_str("(double 5)");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment {
        bindings: hashmap! {
            "double".to_string() => yeti::Expression::NativeFunction(
              |env, args| {
                let (env, args) = yeti::evaluate_expressions(env, args)?;
                match &args[0] {
                  yeti::Expression::Integer(i) => Ok((env, yeti::Expression::Integer(i * Integer::from(2)))),
                  _ => panic!("Expected integer argument"),
                }
              }
            ),
        },
        servers: Arc::new(Mutex::new(HashMap::new())),
    };
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_add() -> Result {
    let tokens = yeti::Tokens::from_str("(+ 5 3)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_if_then_branch() -> Result {
    let tokens = yeti::Tokens::from_str("(if true 1 2)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_if_else_branch() -> Result {
    let tokens = yeti::Tokens::from_str("(if false 1 2)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_def() -> Result {
    let tokens = yeti::Tokens::from_str("(def x 5)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (actual_environment, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert(
        "x".to_string(),
        yeti::Expression::Integer(Integer::from(5)),
    );
    assert_eq!(actual_environment.bindings, expected_environment.bindings);
    Ok(())
}

#[test]
fn evaluate_array() -> Result {
    let tokens = yeti::Tokens::from_str("[(+ 1 2) (/ 4 3)]");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Array(vector![
        yeti::Expression::Integer(Integer::from(3)),
        yeti::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map() -> Result {
    let tokens = yeti::Tokens::from_str("{:a (+ 1 2) :b (/ 4 3)}");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Map(hashmap! {
        yeti::Expression::Keyword(":a".to_string()) => yeti::Expression::Integer(Integer::from(3)),
        yeti::Expression::Keyword(":b".to_string()) => yeti::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_quote() -> Result {
    let tokens = yeti::Tokens::from_str("'(1 2)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Call {
        function: Box::new(yeti::Expression::Integer(Integer::from(1))),
        arguments: vector![yeti::Expression::Integer(Integer::from(2)),],
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_eval() -> Result {
    let tokens = yeti::Tokens::from_str("(eval '(+ 1 2))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_read_string() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(read-string "(+ 1 2)")"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Call {
        function: Box::new(yeti::Expression::Symbol("+".to_string())),
        arguments: vector![
            yeti::Expression::Integer(Integer::from(1)),
            yeti::Expression::Integer(Integer::from(2)),
        ],
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_fn() -> Result {
    let tokens = yeti::Tokens::from_str("(fn [x] (* x 2))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Function {
        parameters: vector![yeti::Expression::Symbol("x".to_string()),],
        body: Box::new(yeti::Expression::Call {
            function: Box::new(yeti::Expression::Symbol("*".to_string())),
            arguments: vector![
                yeti::Expression::Symbol("x".to_string()),
                yeti::Expression::Integer(Integer::from(2)),
            ],
        }),
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_call_fn() -> Result {
    let tokens = yeti::Tokens::from_str("((fn [x] (* x 2)) 5)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_defn() -> Result {
    let tokens = yeti::Tokens::from_str("(defn double [x] (* x 2))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (actual_environment, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert(
        "double".to_string(),
        yeti::Expression::Function {
            parameters: vector![yeti::Expression::Symbol("x".to_string()),],
            body: Box::new(yeti::Expression::Call {
                function: Box::new(yeti::Expression::Symbol("*".to_string())),
                arguments: vector![
                    yeti::Expression::Symbol("x".to_string()),
                    yeti::Expression::Integer(Integer::from(2)),
                ],
            }),
        },
    );
    assert_eq!(actual_environment.bindings, expected_environment.bindings);
    Ok(())
}

#[test]
fn evaluate_multiply_ratio_by_integer() -> Result {
    let tokens = yeti::Tokens::from_str("(* 7/3 3)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_multiply_integer_by_ratio() -> Result {
    let tokens = yeti::Tokens::from_str("(* 3 7/3)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_equality_when_true() -> Result {
    let tokens = yeti::Tokens::from_str("(= 3 3)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_equality_when_false() -> Result {
    let tokens = yeti::Tokens::from_str("(= 3 4)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Bool(false);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_equality_of_floats() -> Result {
    let tokens = yeti::Tokens::from_str("(= 3.4 3.4)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}
