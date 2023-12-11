use base;
use compiler;
use rug::{Integer, Rational};
use im::{vector, ordmap};

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn add_two_integers() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(+ 2 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn add_three_two_integers() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(+ 2 3 4)").await?;
    let expected = compiler::Expression::Integer(Integer::from(9));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_array() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "[(+ 1 2) (/ 4 3)]").await?;
    let expected = compiler::Expression::Array(vector![
        compiler::Expression::Integer(Integer::from(3)),
        compiler::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_map() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "{:a (+ 1 2) :b (/ 4 3)}").await?;
    let expected = compiler::Expression::Map(ordmap! {
        compiler::Expression::Keyword(":a".to_string()) => compiler::Expression::Integer(Integer::from(3)),
        compiler::Expression::Keyword(":b".to_string()) => compiler::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3))))
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_multiply_ratio_by_integer() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(* 7/3 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_multiply_integer_by_ratio() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(* 3 7/3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_when_true() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(= 3 3)").await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_when_false() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(= 3 4)").await?;
    let expected = compiler::Expression::Bool(false);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_equality_of_floats() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "(= 3.4 3.4)").await?;
    let expected = compiler::Expression::Bool(true);
    assert_eq!(actual, expected);
    Ok(())
}

