use forge;
use rug::Integer;

type Result = std::result::Result<(), forge::RaisedEffect>;

#[test]
fn get_key_from_map() -> Result {
    let tokens = forge::Tokens::from_str("(get {:a 1} :a)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn get_non_existing_key_from_map() -> Result {
    let tokens = forge::Tokens::from_str("(get {:a 1} :b)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn get_non_existing_key_from_map_with_default_value() -> Result {
    let tokens = forge::Tokens::from_str("(get {:a 1} :b 5)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}
