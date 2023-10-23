use forge;
use im::hashmap;
use rug::Integer;

type Result = std::result::Result<(), forge::effect::Effect>;

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

#[test]
fn evaluate_key_on_map() -> Result {
    let tokens = forge::Tokens::from_str("(:a {:a 1})");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map_on_key() -> Result {
    let tokens = forge::Tokens::from_str("({:a 1} :a)");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map_on_key_non_keyword() -> Result {
    let tokens = forge::Tokens::from_str("({1 :a} 1)");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Keyword(":a".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_assoc() -> Result {
    let tokens = forge::Tokens::from_str("(assoc {} :a 1)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(1)),
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_dissoc() -> Result {
    let tokens = forge::Tokens::from_str("(dissoc {:a 1} :a)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {});
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_merge() -> Result {
    let tokens = forge::Tokens::from_str("(merge {:a 1} {:b 2})");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(1)),
        forge::Expression::Keyword(":b".to_string()) => forge::Expression::Integer(Integer::from(2)),
    });
    assert_eq!(actual, expected);
    Ok(())
}
