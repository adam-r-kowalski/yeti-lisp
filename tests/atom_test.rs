use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn deref_an_atom() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def x (atom 5))").await?;
    let (_, actual) = yeti::evaluate_source(env, "@x").await?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn reset_an_atom() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def x (atom 5))").await?;
    let (env, previous) = yeti::evaluate_source(env, "@x").await?;
    let (env, _) = yeti::evaluate_source(env, "(reset! x 10)").await?;
    let (_, actual) = yeti::evaluate_source(env, "@x").await?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(previous, expected);
    Ok(())
}

#[tokio::test]
async fn swap_an_atom() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def x (atom 5))").await?;
    let (env, previous) = yeti::evaluate_source(env, "@x").await?;
    let (env, _) = yeti::evaluate_source(env, "(swap! x inc)").await?;
    let (_, actual) = yeti::evaluate_source(env, "@x").await?;
    let expected = yeti::Expression::Integer(Integer::from(6));
    assert_eq!(actual, expected);
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(previous, expected);
    Ok(())
}
