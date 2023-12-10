use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn add_two_integers() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(+ 2 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn add_three_two_integers() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(+ 2 3 4)").await?;
    let expected = compiler::Expression::Integer(Integer::from(9));
    assert_eq!(actual, expected);
    Ok(())
}
