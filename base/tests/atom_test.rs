use base;
use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn deref_an_atom() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(env, "(def x (atom 5))").await?;
    let (_, actual) = compiler::evaluate_source(env, "@x").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn reset_an_atom() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(env, "(def x (atom 5))").await?;
    let (env, previous) = compiler::evaluate_source(env, "@x").await?;
    let (env, _) = compiler::evaluate_source(env, "(reset! x 10)").await?;
    let (_, actual) = compiler::evaluate_source(env, "@x").await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(previous, expected);
    Ok(())
}

#[tokio::test]
async fn swap_an_atom() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(env, "(def x (atom 5))").await?;
    let (env, previous) = compiler::evaluate_source(env, "@x").await?;
    let (env, _) = compiler::evaluate_source(env, "(swap! x inc)").await?;
    let (_, actual) = compiler::evaluate_source(env, "@x").await?;
    let expected = compiler::Expression::Integer(Integer::from(6));
    assert_eq!(actual, expected);
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(previous, expected);
    Ok(())
}
