use compiler;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn check_if_symbol_is_bound() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, "(bound? x)").await?;
    let expected = compiler::Expression::Bool(false);
    assert_eq!(actual, expected);
    let (env, _) = compiler::evaluate_source(env, "(def x 5)").await?;
    let (_, actual) = compiler::evaluate_source(env, "(bound? x)").await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}
