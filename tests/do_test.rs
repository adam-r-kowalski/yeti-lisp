use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn do_returns_the_3rd_expression() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(do 5 3 true)")?;
    let expected = yeti::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn do_returns_the_2nd_expression() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(do 5 3)")?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn do_returns_the_1st_expression() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(do 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn do_returns_nil() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(do)")?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
