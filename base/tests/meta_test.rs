use base;
use compiler;
use compiler::expression::Call;
use im::vector;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn check_if_symbol_is_bound() -> Result {
    let env = base::environment();
    let (env, actual) = compiler::evaluate_source(env, "(bound? x)").await?;
    let expected = compiler::Expression::Bool(false);
    assert_eq!(actual, expected);
    let (env, _) = compiler::evaluate_source(env, "(def x 5)").await?;
    let (_, actual) = compiler::evaluate_source(env, "(bound? x)").await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_eval() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(eval '(+ 1 2))").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_read_string() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, r#"(read-string "(+ 1 2)")"#).await?;
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
async fn evaluate_assert_true_does_nothing() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(assert true)").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_assert_false_raises_error() -> Result {
    let env = base::environment();
    let result = compiler::evaluate_source(env, "(assert false)").await;
    assert!(result.is_err());
    Ok(())
}
