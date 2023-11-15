use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn nth_inbounds_gives_values() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(nth [1 2 3] 0)").await?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn nth_out_of_bounds_gives_error() -> Result {
    let env = yeti::core::environment();
    let actual = yeti::evaluate_source(env, "(nth [1 2 3] 4)").await;
    let expected = yeti::effect::error("Index out of range");
    assert_eq!(actual, Err(expected));
    Ok(())
}

#[tokio::test]
async fn nth_out_of_bounds_with_default_gives_default() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(nth [1 2 3] 4 100)").await?;
    let expected = yeti::Expression::Integer(Integer::from(100));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn count_of_array() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(count [1 4 9])").await?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}
