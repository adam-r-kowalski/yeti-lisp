use compiler;
use compiler::expression::Call;
use im::{ordmap, vector};
use rug::{Integer, Rational};

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn evaluate_keyword() -> Result {
    let tokens = compiler::Tokens::from_str(":x");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_string() -> Result {
    let tokens = compiler::Tokens::from_str(r#""hello""#);
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_integer() -> Result {
    let tokens = compiler::Tokens::from_str("5");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_float() -> Result {
    let tokens = compiler::Tokens::from_str("3.14");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Float(compiler::Float::from_str("3.14"));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_symbol_bound_to_integer() -> Result {
    let tokens = compiler::Tokens::from_str("x");
    let expression = compiler::parse(tokens);
    let environment = ordmap! {
        "x".to_string() => compiler::Expression::Integer(Integer::from(5))
    };
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_symbol_bound_to_function() -> Result {
    let tokens = compiler::Tokens::from_str("(double 5)");
    let expression = compiler::parse(tokens);
    let environment = ordmap! {
        "double".to_string() => compiler::Expression::NativeFunction(
          |env, args| {
            Box::pin(async move {
                let (env, args) = compiler::evaluate_expressions(env, args).await?;
                match &args[0] {
                  compiler::Expression::Integer(i) => Ok((env, compiler::Expression::Integer(i * Integer::from(2)))),
                  _ => panic!("Expected integer argument"),
                }
            })
          }
        )
    };
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_add() -> Result {
    let tokens = compiler::Tokens::from_str("(+ 5 3)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_if_then_branch() -> Result {
    let tokens = compiler::Tokens::from_str("(if true 1 2)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_if_else_branch() -> Result {
    let tokens = compiler::Tokens::from_str("(if false 1 2)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_def() -> Result {
    let tokens = compiler::Tokens::from_str("(def x 5)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (actual_environment, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert(
        "x".to_string(),
        compiler::Expression::Integer(Integer::from(5)),
    );
    assert_eq!(actual_environment, expected_environment);
    Ok(())
}

#[tokio::test]
async fn evaluate_array() -> Result {
    let tokens = compiler::Tokens::from_str("[(+ 1 2) (/ 4 3)]");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Array(vector![
        compiler::Expression::Integer(Integer::from(3)),
        compiler::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map() -> Result {
    let tokens = compiler::Tokens::from_str("{:a (+ 1 2) :b (/ 4 3)}");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Map(ordmap! {
        compiler::Expression::Keyword(":a".to_string()) => compiler::Expression::Integer(Integer::from(3)),
        compiler::Expression::Keyword(":b".to_string()) => compiler::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3))))
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_quote() -> Result {
    let tokens = compiler::Tokens::from_str("'(1 2)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Call(Call {
        function: Box::new(compiler::Expression::Integer(Integer::from(1))),
        arguments: vector![compiler::Expression::Integer(Integer::from(2)),],
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_eval() -> Result {
    let tokens = compiler::Tokens::from_str("(eval '(+ 1 2))");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_read_string() -> Result {
    let tokens = compiler::Tokens::from_str(r#"(read-string "(+ 1 2)")"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Call(Call {
        function: Box::new(compiler::Expression::Symbol("+".to_string())),
        arguments: vector![
            compiler::Expression::Integer(Integer::from(1)),
            compiler::Expression::Integer(Integer::from(2)),
        ],
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_fn() -> Result {
    let tokens = compiler::Tokens::from_str("(fn [x] (* x 2))");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Function(compiler::expression::Function {
        env: environment.clone(),
        patterns: vector![compiler::expression::Pattern {
            parameters: vector![compiler::Expression::Symbol("x".to_string()),],
            body: vector![compiler::Expression::Call(Call {
                function: Box::new(compiler::Expression::Symbol("*".to_string())),
                arguments: vector![
                    compiler::Expression::Symbol("x".to_string()),
                    compiler::Expression::Integer(Integer::from(2)),
                ],
            })],
        }],
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_call_fn() -> Result {
    let tokens = compiler::Tokens::from_str("((fn [x] (* x 2)) 5)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_defn() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, "(defn double [x] (* x 2))").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    let (_, actual) = compiler::evaluate_source(env, "(double 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_multiply_ratio_by_integer() -> Result {
    let tokens = compiler::Tokens::from_str("(* 7/3 3)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_multiply_integer_by_ratio() -> Result {
    let tokens = compiler::Tokens::from_str("(* 3 7/3)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_when_true() -> Result {
    let tokens = compiler::Tokens::from_str("(= 3 3)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_when_false() -> Result {
    let tokens = compiler::Tokens::from_str("(= 3 4)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Bool(false);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_of_floats() -> Result {
    let tokens = compiler::Tokens::from_str("(= 3.4 3.4)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_assert_true_does_nothing() -> Result {
    let tokens = compiler::Tokens::from_str("(assert true)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_assert_false_raises_error() -> Result {
    let tokens = compiler::Tokens::from_str("(assert false)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let result = compiler::evaluate(environment.clone(), expression).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn evaluate_str_concat() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, r#"(str "hello" " " "world")"#).await?;
    let (_, expected) = compiler::evaluate_source(env, r#""hello world""#).await?;
    assert_eq!(actual, expected);
    Ok(())
}
