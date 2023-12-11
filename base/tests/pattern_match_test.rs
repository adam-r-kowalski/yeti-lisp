use compiler;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn pattern_match_array() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [[x y]] x) [1 2])").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_map() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [{:a a}] a) {:a 5})").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_map_in_array() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [[_ {:a a}]] a) [0 {:a 7}])").await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_array_with_literal_keyword() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [[:foo y]] y) [:foo 2])").await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_array_with_literal_string() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, r#"((fn [["foo" y]] y) ["foo" 2])"#).await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_array_with_literal_integer() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, r#"((fn [[7 y]] y) [7 2])"#).await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_array_with_literal_nil() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [[nil y]] y) [nil 2])").await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_multiple_patterns_first_taken() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(
        env,
        r#"
        ((fn
          ([:apple] "you picked apple")
          ([:mango] "you selected mango"))
          :apple)
        "#,
    )
    .await?;
    let expected = compiler::Expression::String("you picked apple".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn pattern_match_multiple_patterns_second_taken() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(
        env,
        r#"
        ((fn
          ([:apple] "you picked apple")
          ([:mango] "you selected mango"))
          :mango)
        "#,
    )
    .await?;
    let expected = compiler::Expression::String("you selected mango".to_string());
    assert_eq!(actual, expected);
    Ok(())
}
