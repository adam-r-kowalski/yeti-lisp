use base;
use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn evaluate_def() -> Result {
    let env = base::environment();
    let (actual_env, actual) = compiler::evaluate_source(env.clone(), "(def x 5)").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_env = env;
    expected_env.insert(
        "x".to_string(),
        compiler::Expression::Integer(Integer::from(5)),
    );
    assert_eq!(actual_env, expected_env);
    Ok(())
}
