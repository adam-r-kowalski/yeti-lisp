use compiler;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn for_binding() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, "(for [x [1 2 3]] (* x x))").await?;
    let (_, expected) = compiler::evaluate_source(env, "[1 4 9]").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn for_binding_with_pattern_matching() -> Result {
    let env = compiler::core::environment();
    let (env, actual) =
        compiler::evaluate_source(env, "(for [{:a x} [{:a 1} {:a 2} {:a 3}]] (* x x))").await?;
    let (_, expected) = compiler::evaluate_source(env, "[1 4 9]").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn for_binding_with_multiple_expressions_returns_last_one() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, "(for [x [1 2 3]] (* x x) true)").await?;
    let (_, expected) = compiler::evaluate_source(env, "[true true true]").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn for_over_range() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(env, "(for [x (range 5)] (* x x))").await?;
    let (_, expected) = compiler::evaluate_source(env, "[0 1 4 9 16]").await?;
    assert_eq!(actual, expected);
    Ok(())
}
