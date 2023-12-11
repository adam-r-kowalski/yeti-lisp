use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn do_returns_the_3rd_expression() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(do 5 3 true)").await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn do_returns_the_2nd_expression() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(do 5 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn do_returns_the_1st_expression() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(do 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn do_returns_nil() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(do)").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
