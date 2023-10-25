use yeti;
use rug::Integer;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn nth_inbounds_gives_values() -> Result {
    let tokens = yeti::Tokens::from_str("(nth [1 2 3] 0)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn nth_out_of_bounds_gives_error() -> Result {
    let tokens = yeti::Tokens::from_str("(nth [1 2 3] 4)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let actual = yeti::evaluate(environment, expression).err().unwrap();
    let expected = yeti::effect::error("Index out of range");
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn nth_out_of_bounds_with_default_gives_default() -> Result {
    let tokens = yeti::Tokens::from_str("(nth [1 2 3] 4 100)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(100));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn count_of_array() -> Result {
    let tokens = yeti::Tokens::from_str("(count [1 4 9])");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}
