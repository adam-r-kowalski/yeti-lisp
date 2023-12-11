use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn thread_first() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(-> 5 (- 3))").await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn thread_first_multiple_threads() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(-> 5 (- 3) (- 4))").await?;
    let expected = compiler::Expression::Integer(Integer::from(-2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn thread_first_wraps_in_call_if_not_already() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(-> 5 inc inc)").await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}
