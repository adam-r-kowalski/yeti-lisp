use compiler;
use rug::{Integer, Rational};

#[test]
fn tokenize_symbol() {
    let actual = compiler::tokenize(
        "snake_case PascalCase kebab-case camelCase predicate? -> namespaced/symbol",
    );
    let expected = vec![
        compiler::Token::Symbol("snake_case".to_string()),
        compiler::Token::Symbol("PascalCase".to_string()),
        compiler::Token::Symbol("kebab-case".to_string()),
        compiler::Token::Symbol("camelCase".to_string()),
        compiler::Token::Symbol("predicate?".to_string()),
        compiler::Token::Symbol("->".to_string()),
        compiler::Token::NamespacedSymbol(vec!["namespaced".to_string(), "symbol".to_string()]),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_keyword() {
    let actual = compiler::tokenize(
        ":snake_case :PascalCase :kebab-case :camelCase :predicate? :that's",
    );
    let expected = vec![
        compiler::Token::Keyword(":snake_case".to_string()),
        compiler::Token::Keyword(":PascalCase".to_string()),
        compiler::Token::Keyword(":kebab-case".to_string()),
        compiler::Token::Keyword(":camelCase".to_string()),
        compiler::Token::Keyword(":predicate?".to_string()),
        compiler::Token::Keyword(":that's".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_string_literal() {
    let actual = compiler::tokenize(
        r#""hello" "world" "123" "that's" "that’s" "Quoted \"String\"""#,
    );
    let expected = vec![
        compiler::Token::String("hello".to_string()),
        compiler::Token::String("world".to_string()),
        compiler::Token::String("123".to_string()),
        compiler::Token::String("that's".to_string()),
        compiler::Token::String("that’s".to_string()),
        compiler::Token::String("Quoted \"String\"".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_integer() {
    let actual =
        compiler::tokenize("123 456 789 1_000 -321 -456");
    let expected = vec![
        compiler::Token::Integer(Integer::from(123)),
        compiler::Token::Integer(Integer::from(456)),
        compiler::Token::Integer(Integer::from(789)),
        compiler::Token::Integer(Integer::from(1000)),
        compiler::Token::Integer(Integer::from(-321)),
        compiler::Token::Integer(Integer::from(-456)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_float() {
    let actual = compiler::tokenize("1.23 4.56 7.89 1_000.0 -3.23");
    let expected = vec![
        compiler::Token::Float(compiler::Float::from_str("1.23")),
        compiler::Token::Float(compiler::Float::from_str("4.56")),
        compiler::Token::Float(compiler::Float::from_str("7.89")),
        compiler::Token::Float(compiler::Float::from_str("1000.0")),
        compiler::Token::Float(compiler::Float::from_str("-3.23")),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_delimiters() {
    let actual = compiler::tokenize("( { [ ] } )");
    let expected = vec![
        compiler::Token::LeftParen,
        compiler::Token::LeftBrace,
        compiler::Token::LeftBracket,
        compiler::Token::RightBracket,
        compiler::Token::RightBrace,
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_call_inside_array() {
    let actual = compiler::tokenize("[3.14 (+ 2 3)]");
    let expected = vec![
        compiler::Token::LeftBracket,
        compiler::Token::Float(compiler::Float::from_str("3.14")),
        compiler::Token::LeftParen,
        compiler::Token::Symbol("+".to_string()),
        compiler::Token::Integer(Integer::from(2)),
        compiler::Token::Integer(Integer::from(3)),
        compiler::Token::RightParen,
        compiler::Token::RightBracket,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_quote() {
    let actual = compiler::tokenize("'(1 2)");
    let expected = vec![
        compiler::Token::Quote,
        compiler::Token::LeftParen,
        compiler::Token::Integer(Integer::from(1)),
        compiler::Token::Integer(Integer::from(2)),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_ratio() {
    let actual = compiler::tokenize("5/3 4/2");
    let expected = vec![
        compiler::Token::Ratio(Rational::from((Integer::from(5), Integer::from(3)))),
        compiler::Token::Integer(Integer::from(2)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_deref() {
    let actual = compiler::tokenize("@ @x @(atom x)");
    let expected = vec![
        compiler::Token::Deref,
        compiler::Token::Deref,
        compiler::Token::Symbol("x".to_string()),
        compiler::Token::Deref,
        compiler::Token::LeftParen,
        compiler::Token::Symbol("atom".to_string()),
        compiler::Token::Symbol("x".to_string()),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_comment_after_expression() {
    let actual = compiler::tokenize("(+ 1 2) ; comment after expression");
    let expected = vec![
        compiler::Token::LeftParen,
        compiler::Token::Symbol("+".to_string()),
        compiler::Token::Integer(Integer::from(1)),
        compiler::Token::Integer(Integer::from(2)),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_comment_before_expression() {
    let actual = compiler::tokenize(
        r#"
          ; comment before expression
          (+ 1 2)
        "#,
    );
    let expected = vec![
        compiler::Token::LeftParen,
        compiler::Token::Symbol("+".to_string()),
        compiler::Token::Integer(Integer::from(1)),
        compiler::Token::Integer(Integer::from(2)),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_comment_in_between_expression() {
    let actual = compiler::tokenize(
        r#"
          (+ 1 ; comment before expression
             2)
        "#,
    );
    let expected = vec![
        compiler::Token::LeftParen,
        compiler::Token::Symbol("+".to_string()),
        compiler::Token::Integer(Integer::from(1)),
        compiler::Token::Integer(Integer::from(2)),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_paren_after_keyword() {
    let actual = compiler::tokenize("(get map :key)");
    let expected = vec![
        compiler::Token::LeftParen,
        compiler::Token::Symbol("get".to_string()),
        compiler::Token::Symbol("map".to_string()),
        compiler::Token::Keyword(":key".to_string()),
        compiler::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}
