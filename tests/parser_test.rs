use im::vector;
use rug::Integer;
use tao;

#[test]
fn parse_integer() {
    let tokens = tao::tokenize("123");
    let actual = tao::parse(tokens);
    let expected = tao::Expression::Integer(Integer::from(123));
    assert_eq!(actual, expected);
}

#[test]
fn parse_float() {
    let tokens = tao::tokenize("3.14");
    let actual = tao::parse(tokens);
    let expected = tao::Expression::Float(tao::string_to_float("3.14"));
    assert_eq!(actual, expected);
}

#[test]
fn parse_array_of_integer() {
    let tokens = tao::tokenize("[1 2 3]");
    let actual = tao::parse(tokens);
    let expected = tao::Expression::Array(vector![
        tao::Expression::Integer(Integer::from(1)),
        tao::Expression::Integer(Integer::from(2)),
        tao::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_array_of_float_and_integer() {
    let tokens = tao::tokenize("[3.14 2 3]");
    let actual = tao::parse(tokens);
    let expected = tao::Expression::Array(vector![
        tao::Expression::Float(tao::string_to_float("3.14")),
        tao::Expression::Integer(Integer::from(2)),
        tao::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}
