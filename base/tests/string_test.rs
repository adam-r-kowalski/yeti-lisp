use compiler;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn evaluate_str_concat() -> Result {
    let env = base::environment();
    let (env, actual) = compiler::evaluate_source(env, r#"(str "hello" " " "world")"#).await?;
    let (_, expected) = compiler::evaluate_source(env, r#""hello world""#).await?;
    assert_eq!(actual, expected);
    Ok(())
}
