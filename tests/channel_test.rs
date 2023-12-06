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

#[tokio::test]
async fn take_then_put_channel() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def c (chan))").await?;
    let (env, _) = yeti::evaluate_source(env, "(spawn (take! c))").await?;
    let (_, actual) = yeti::evaluate_source(env, r#"(put! c "hello channel")"#).await?;
    assert!(matches!(actual, yeti::Expression::Nil));
    Ok(())
}

#[tokio::test]
async fn channel_with_specified_buffer_size() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def c (chan 3))").await?;
    let (env, _) = yeti::evaluate_source(env, "(put! c 1)").await?;
    let (env, _) = yeti::evaluate_source(env, "(put! c 2)").await?;
    let (env, _) = yeti::evaluate_source(env, "(put! c 3)").await?;
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    let (env, expected) = yeti::evaluate_source(env, "1").await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    let (env, expected) = yeti::evaluate_source(env, "2").await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    let (_, expected) = yeti::evaluate_source(env, "3").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn closing_a_channel() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def c (chan 3))").await?;
    let (env, _) = yeti::evaluate_source(env, "(put! c nil)").await?;
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::Nil);
    let (_, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::Nil);
    Ok(())
}
