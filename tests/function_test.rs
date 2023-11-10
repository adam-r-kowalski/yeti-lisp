use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn function_definition_and_call() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(defn double [x] (* x 2))")?;
    let (env, actual) = yeti::evaluate_source(env, "(double 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    let actual = yeti::evaluate_source(env, "x");
    assert!(matches!(actual, Err(yeti::effect::Effect::Error(_))));
    Ok(())
}

#[test]
fn multi_line_function() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
         (defn sum-of-squares [x y]
          (def x2 (* x x))
          (def y2 (* y y))
          (+ x2 y2))
        "#,
    )?;
    let (_, actual) = yeti::evaluate_source(env, "(sum-of-squares 5 7)")?;
    let expected = yeti::Expression::Integer(Integer::from(74));
    assert_eq!(actual, expected);
    Ok(())
}
