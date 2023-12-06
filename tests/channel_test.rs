use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn create_a_channel() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(chan)").await?;
    assert!(matches!(actual, yeti::Expression::Channel(_)));
    Ok(())
}

#[tokio::test]
async fn put_and_take_off_channel() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def c (chan))").await?;
    let (env, actual) = yeti::evaluate_source(env, r#"(put! c "hello channel")"#).await?;
    assert!(matches!(actual, yeti::Expression::Nil));
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    let (_, expected) = yeti::evaluate_source(env, r#""hello channel""#).await?;
    assert_eq!(actual, expected);
    Ok(())
}
