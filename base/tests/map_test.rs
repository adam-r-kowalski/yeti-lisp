use compiler;
use im::ordmap;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn get_key_from_map() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(get {:a 1} :a)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn get_non_existing_key_from_map() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(get {:a 1} :b)").await?;
    let expected = compiler::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn get_non_existing_key_from_map_with_default_value() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(get {:a 1} :b 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_key_on_map() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(:a {:a 1})").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map_on_key() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "({:a 1} :a)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map_on_key_non_keyword() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "({1 :a} 1)").await?;
    let expected = compiler::Expression::Keyword(":a".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_assoc() -> Result {
    let env = base::environment();
    let (env, actual) = compiler::evaluate_source(env, "(assoc {} :a 1)").await?;
    let (_, expected) = compiler::evaluate_source(env, "{:a 1}").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_dissoc() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(dissoc {:a 1} :a)").await?;
    let expected = compiler::Expression::Map(ordmap! {});
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_merge() -> Result {
    let env = base::environment();
    let (env, actual) = compiler::evaluate_source(env, "(merge {:a 1} {:b 2})").await?;
    let (_, expected) = compiler::evaluate_source(env, "{:a 1 :b 2}").await?;
    assert_eq!(actual, expected);
    Ok(())
}
