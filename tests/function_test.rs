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
