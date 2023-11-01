use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn let_binding() -> Result {
    let tokens = yeti::Tokens::from_str("(let [a 5] a)");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn two_let_binding() -> Result {
    let tokens = yeti::Tokens::from_str("(let [a 5 b 10] (+ a b))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(15));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn let_binding_with_pattern_match() -> Result {
    let tokens = yeti::Tokens::from_str("(let [[x y] [1 2]] (+ x y))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}
