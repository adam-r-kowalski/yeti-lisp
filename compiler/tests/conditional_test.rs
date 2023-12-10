use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn if_when_condition_is_true() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(if true 5 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn if_when_condition_is_false() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(if false 5 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn when_if_condition_is_true() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(when true 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn when_if_condition_is_false() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(when false 5)").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
