/*
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn for_binding() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(env, "(for [x [1 2 3]] (* x x))")?;
    let (_, expected) = yeti::evaluate_source(env, "[1 4 9]")?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn for_binding_with_pattern_matching() -> Result {
    let env = yeti::core::environment();
    let (env, actual) =
        yeti::evaluate_source(env, "(for [{:a x} [{:a 1} {:a 2} {:a 3}]] (* x x))")?;
    let (_, expected) = yeti::evaluate_source(env, "[1 4 9]")?;
    assert_eq!(actual, expected);
    Ok(())
}
*/
