use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn check_if_symbol_is_bound() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(env, "(bound? x)").await?;
    let expected = yeti::Expression::Bool(false);
    assert_eq!(actual, expected);
    let (env, _) = yeti::evaluate_source(env, "(def x 5)").await?;
    let (_, actual) = yeti::evaluate_source(env, "(bound? x)").await?;
    let expected = yeti::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}
