use rug::Integer;
use tao;

#[test]
fn tokenize_symbol() {
    let actual = tao::tokenize("snake_case PascalCase kebab-case camelCase predicate?");
    let expected = vec![
        tao::Token::Symbol("snake_case".to_string()),
        tao::Token::Symbol("PascalCase".to_string()),
        tao::Token::Symbol("kebab-case".to_string()),
        tao::Token::Symbol("camelCase".to_string()),
        tao::Token::Symbol("predicate?".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_keyword() {
    let actual = tao::tokenize(":snake_case :PascalCase :kebab-case :camelCase :predicate?");
    let expected = vec![
        tao::Token::Keyword(":snake_case".to_string()),
        tao::Token::Keyword(":PascalCase".to_string()),
        tao::Token::Keyword(":kebab-case".to_string()),
        tao::Token::Keyword(":camelCase".to_string()),
        tao::Token::Keyword(":predicate?".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_string_literal() {
    let actual = tao::tokenize(r#""hello" "world" "123""#);
    let expected = vec![
        tao::Token::String("hello".to_string()),
        tao::Token::String("world".to_string()),
        tao::Token::String("123".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_integer() {
    let actual = tao::tokenize("123 456 789 1_000 -321 -456");
    let expected = vec![
        tao::Token::Integer(Integer::from(123)),
        tao::Token::Integer(Integer::from(456)),
        tao::Token::Integer(Integer::from(789)),
        tao::Token::Integer(Integer::from(1000)),
        tao::Token::Integer(Integer::from(-321)),
        tao::Token::Integer(Integer::from(-456)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_float() {
    let actual = tao::tokenize("1.23 4.56 7.89 1_000.0 -3.23");
    let expected = vec![
        tao::Token::Float(tao::string_to_float("1.23")),
        tao::Token::Float(tao::string_to_float("4.56")),
        tao::Token::Float(tao::string_to_float("7.89")),
        tao::Token::Float(tao::string_to_float("1000.0")),
        tao::Token::Float(tao::string_to_float("-3.23")),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_delimiters() {
    let actual = tao::tokenize("( { [ ] } )");
    let expected = vec![
        tao::Token::LeftParen,
        tao::Token::LeftBrace,
        tao::Token::LeftBracket,
        tao::Token::RightBracket,
        tao::Token::RightBrace,
        tao::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

