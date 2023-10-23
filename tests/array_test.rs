use forge;
use rug::Integer;

type Result = std::result::Result<(), forge::effect::Effect>;

#[test]
fn nth_inbounds_gives_values() -> Result {
    let tokens = forge::Tokens::from_str("(nth [1 2 3] 0)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn nth_out_of_bounds_gives_error() -> Result {
    let tokens = forge::Tokens::from_str("(nth [1 2 3] 4)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let actual = forge::evaluate(environment, expression).err().unwrap();
    let expected = forge::effect::error("Index out of range");
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn nth_out_of_bounds_with_default_gives_default() -> Result {
    let tokens = forge::Tokens::from_str("(nth [1 2 3] 4 100)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(100));
    assert_eq!(actual, expected);
    Ok(())
}
