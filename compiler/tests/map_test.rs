use compiler;
use im::ordmap;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn get_key_from_map() -> Result {
    let tokens = compiler::Tokens::from_str("(get {:a 1} :a)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn get_non_existing_key_from_map() -> Result {
    let tokens = compiler::Tokens::from_str("(get {:a 1} :b)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn get_non_existing_key_from_map_with_default_value() -> Result {
    let tokens = compiler::Tokens::from_str("(get {:a 1} :b 5)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_key_on_map() -> Result {
    let tokens = compiler::Tokens::from_str("(:a {:a 1})");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map_on_key() -> Result {
    let tokens = compiler::Tokens::from_str("({:a 1} :a)");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map_on_key_non_keyword() -> Result {
    let tokens = compiler::Tokens::from_str("({1 :a} 1)");
    let expression = compiler::parse(tokens);
    let environment = compiler::Environment::new();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Keyword(":a".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_assoc() -> Result {
    let tokens = compiler::Tokens::from_str("(assoc {} :a 1)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Map(ordmap! {
        compiler::Expression::Keyword(":a".to_string()) => compiler::Expression::Integer(Integer::from(1))
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_dissoc() -> Result {
    let tokens = compiler::Tokens::from_str("(dissoc {:a 1} :a)");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Map(ordmap! {});
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_merge() -> Result {
    let tokens = compiler::Tokens::from_str("(merge {:a 1} {:b 2})");
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::Map(ordmap! {
        compiler::Expression::Keyword(":a".to_string()) => compiler::Expression::Integer(Integer::from(1)),
        compiler::Expression::Keyword(":b".to_string()) => compiler::Expression::Integer(Integer::from(2))
    });
    assert_eq!(actual, expected);
    Ok(())
}
