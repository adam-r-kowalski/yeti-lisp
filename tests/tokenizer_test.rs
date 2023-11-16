use rug::{Integer, Rational};
use yeti;

#[test]
fn tokenize_symbol() {
    let actual = yeti::Tokens::from_str(
        "snake_case PascalCase kebab-case camelCase predicate? namespaced/symbol",
    )
    .collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Symbol("snake_case".to_string()),
        yeti::Token::Symbol("PascalCase".to_string()),
        yeti::Token::Symbol("kebab-case".to_string()),
        yeti::Token::Symbol("camelCase".to_string()),
        yeti::Token::Symbol("predicate?".to_string()),
        yeti::Token::NamespacedSymbol(vec!["namespaced".to_string(), "symbol".to_string()]),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_keyword() {
    let actual = yeti::Tokens::from_str(
        ":snake_case :PascalCase :kebab-case :camelCase :predicate? :that's",
    )
    .collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Keyword(":snake_case".to_string()),
        yeti::Token::Keyword(":PascalCase".to_string()),
        yeti::Token::Keyword(":kebab-case".to_string()),
        yeti::Token::Keyword(":camelCase".to_string()),
        yeti::Token::Keyword(":predicate?".to_string()),
        yeti::Token::Keyword(":that's".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_string_literal() {
    let actual =
        yeti::Tokens::from_str(r#""hello" "world" "123" "that's" "that’s" "Quoted \"String\"""#)
            .collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::String("hello".to_string()),
        yeti::Token::String("world".to_string()),
        yeti::Token::String("123".to_string()),
        yeti::Token::String("that's".to_string()),
        yeti::Token::String("that’s".to_string()),
        yeti::Token::String("Quoted \"String\"".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_integer() {
    let actual =
        yeti::Tokens::from_str("123 456 789 1_000 -321 -456").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Integer(Integer::from(123)),
        yeti::Token::Integer(Integer::from(456)),
        yeti::Token::Integer(Integer::from(789)),
        yeti::Token::Integer(Integer::from(1000)),
        yeti::Token::Integer(Integer::from(-321)),
        yeti::Token::Integer(Integer::from(-456)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_float() {
    let actual =
        yeti::Tokens::from_str("1.23 4.56 7.89 1_000.0 -3.23").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Float(yeti::Float::from_str("1.23")),
        yeti::Token::Float(yeti::Float::from_str("4.56")),
        yeti::Token::Float(yeti::Float::from_str("7.89")),
        yeti::Token::Float(yeti::Float::from_str("1000.0")),
        yeti::Token::Float(yeti::Float::from_str("-3.23")),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_delimiters() {
    let actual = yeti::Tokens::from_str("( { [ ] } )").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::LeftParen,
        yeti::Token::LeftBrace,
        yeti::Token::LeftBracket,
        yeti::Token::RightBracket,
        yeti::Token::RightBrace,
        yeti::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_call_inside_array() {
    let actual = yeti::Tokens::from_str("[3.14 (+ 2 3)]").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::LeftBracket,
        yeti::Token::Float(yeti::Float::from_str("3.14")),
        yeti::Token::LeftParen,
        yeti::Token::Symbol("+".to_string()),
        yeti::Token::Integer(Integer::from(2)),
        yeti::Token::Integer(Integer::from(3)),
        yeti::Token::RightParen,
        yeti::Token::RightBracket,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_quote() {
    let actual = yeti::Tokens::from_str("'(1 2)").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Quote,
        yeti::Token::LeftParen,
        yeti::Token::Integer(Integer::from(1)),
        yeti::Token::Integer(Integer::from(2)),
        yeti::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_ratio() {
    let actual = yeti::Tokens::from_str("5/3 4/2").collect::<Vec<yeti::Token>>();
    let expected = vec![
        yeti::Token::Ratio(Rational::from((Integer::from(5), Integer::from(3)))),
        yeti::Token::Integer(Integer::from(2)),
    ];
    assert_eq!(actual, expected);
}
