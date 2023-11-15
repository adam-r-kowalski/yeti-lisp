/*
use im::ordmap;
use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn get_key_from_map() -> Result {
    let tokens = yeti::Tokens::from_str("(get {:a 1} :a)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn get_non_existing_key_from_map() -> Result {
    let tokens = yeti::Tokens::from_str("(get {:a 1} :b)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn get_non_existing_key_from_map_with_default_value() -> Result {
    let tokens = yeti::Tokens::from_str("(get {:a 1} :b 5)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_key_on_map() -> Result {
    let tokens = yeti::Tokens::from_str("(:a {:a 1})");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map_on_key() -> Result {
    let tokens = yeti::Tokens::from_str("({:a 1} :a)");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map_on_key_non_keyword() -> Result {
    let tokens = yeti::Tokens::from_str("({1 :a} 1)");
    let expression = yeti::parse(tokens);
    let environment = yeti::Environment::new();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Keyword(":a".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_assoc() -> Result {
    let tokens = yeti::Tokens::from_str("(assoc {} :a 1)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Map(ordmap! {
        yeti::Expression::Keyword(":a".to_string()) => yeti::Expression::Integer(Integer::from(1))
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_dissoc() -> Result {
    let tokens = yeti::Tokens::from_str("(dissoc {:a 1} :a)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Map(ordmap! {});
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_merge() -> Result {
    let tokens = yeti::Tokens::from_str("(merge {:a 1} {:b 2})");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Map(ordmap! {
        yeti::Expression::Keyword(":a".to_string()) => yeti::Expression::Integer(Integer::from(1)),
        yeti::Expression::Keyword(":b".to_string()) => yeti::Expression::Integer(Integer::from(2))
    });
    assert_eq!(actual, expected);
    Ok(())
}
*/
