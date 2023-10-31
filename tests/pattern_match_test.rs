use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn pattern_match_array() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "((fn [[x y]] x) [1 2])")?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_map() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "((fn [{:a a}] a) {:a 5})")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_map_in_array() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "((fn [[_ {:a a}]] a) [0 {:a 7}])")?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_array_with_literal_keyword() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, "((fn [[:foo y]] y) [:foo 2])")?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_array_with_literal_string() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, r#"((fn [["foo" y]] y) ["foo" 2])"#)?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_array_with_literal_integer() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, r#"((fn [[7 y]] y) [7 2])"#)?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_multiple_patterns_first_taken() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(
        env,
        r#"
        ((fn
          ([:apple] "you picked apple")
          ([:mango] "you selected mango"))
          :apple)
        "#,
    )?;
    let expected = yeti::Expression::String("you picked apple".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn pattern_match_multiple_patterns_second_taken() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(
        env,
        r#"
        ((fn
          ([:apple] "you picked apple")
          ([:mango] "you selected mango"))
          :mango)
        "#,
    )?;
    let expected = yeti::Expression::String("you selected mango".to_string());
    assert_eq!(actual, expected);
    Ok(())
}
