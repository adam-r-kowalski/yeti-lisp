use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn let_binding() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "(let [a 5] a)").await?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn two_let_binding() -> Result {
    let tokens = yeti::Tokens::from_str("(let [a 5 b 10] (+ a b))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Integer(Integer::from(15));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn let_binding_with_pattern_match() -> Result {
    let tokens = yeti::Tokens::from_str("(let [[x y] [1 2]] (+ x y))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn let_binding_removes_bindings_after_scope() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(env, "(let [x 5] x)").await?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let result = yeti::evaluate_source(env, "x").await;
    assert!(matches!(result, Err(yeti::effect::Effect::Error(_))));
    Ok(())
}

#[tokio::test]
async fn let_binding_multi_line_body() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(
        env,
        r#"
        (let [x 5]
         (+ 1 2)
         (+ x 2))
        "#,
    )
    .await?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}
