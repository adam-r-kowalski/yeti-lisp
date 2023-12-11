use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn let_binding() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(let [a 5] a)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn two_let_binding() -> Result {
    let tokens = compiler::Tokens::from_str("(let [a 5 b 10] (+ a b))");
    let expression = compiler::parse(tokens);
    let environment = base::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(15));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn let_binding_with_pattern_match() -> Result {
    let tokens = compiler::Tokens::from_str("(let [[x y] [1 2]] (+ x y))");
    let expression = compiler::parse(tokens);
    let environment = base::environment();
    let (_, actual) = compiler::evaluate(environment, expression).await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn let_binding_removes_bindings_after_scope() -> Result {
    let env = base::environment();
    let (env, actual) = compiler::evaluate_source(env, "(let [x 5] x)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let result = compiler::evaluate_source(env, "x").await;
    assert!(matches!(result, Err(compiler::effect::Effect::Error(_))));
    Ok(())
}

#[tokio::test]
async fn let_binding_multi_line_body() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(
        env,
        r#"
        (let [x 5]
         (+ 1 2)
         (+ x 2))
        "#,
    )
    .await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}
