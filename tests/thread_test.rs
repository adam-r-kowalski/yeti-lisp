use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn thread_first() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(-> 5 (- 3))").await?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn thread_first_multiple_threads() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(-> 5 (- 3) (- 4))").await?;
    let expected = yeti::Expression::Integer(Integer::from(-2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn thread_first_wraps_in_call_if_not_already() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(-> 5 inc inc)").await?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}
