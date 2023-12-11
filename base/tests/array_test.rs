use compiler;
use base;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn nth_inbounds_gives_values() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(nth [1 2 3] 0)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn nth_out_of_bounds_gives_error() -> Result {
    let env = base::environment();
    let actual = compiler::evaluate_source(env, "(nth [1 2 3] 4)").await;
    let expected = compiler::effect::error("Index out of range");
    assert_eq!(actual, Err(expected));
    Ok(())
}

#[tokio::test]
async fn nth_out_of_bounds_with_default_gives_default() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(nth [1 2 3] 4 100)").await?;
    let expected = compiler::Expression::Integer(Integer::from(100));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn count_of_array() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(count [1 4 9])").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}
