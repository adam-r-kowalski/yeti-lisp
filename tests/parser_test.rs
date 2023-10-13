use forge;
use im::{hashmap, vector};
use rug::{Integer, Rational};

#[test]
fn parse_symbol() {
    let tokens = forge::Tokens::from_str("x");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Symbol("x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_keyword() {
    let tokens = forge::Tokens::from_str(":x");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_string() {
    let tokens = forge::Tokens::from_str(r#""hello""#);
    let actual = forge::parse(tokens);
    let expected = forge::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_integer() {
    let tokens = forge::Tokens::from_str("123");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Integer(Integer::from(123));
    assert_eq!(actual, expected);
}

#[test]
fn parse_float() {
    let tokens = forge::Tokens::from_str("3.14");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Float(forge::Float::from_str("3.14"));
    assert_eq!(actual, expected);
}

#[test]
fn parse_homogenous_array() {
    let tokens = forge::Tokens::from_str("[1 2 3]");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Array(vector![
        forge::Expression::Integer(Integer::from(1)),
        forge::Expression::Integer(Integer::from(2)),
        forge::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_heterogenous_array() {
    let tokens = forge::Tokens::from_str("[3.14 2 3]");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Array(vector![
        forge::Expression::Float(forge::Float::from_str("3.14")),
        forge::Expression::Integer(Integer::from(2)),
        forge::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_call() {
    let tokens = forge::Tokens::from_str("(+ 1 2)");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Call {
        function: Box::new(forge::Expression::Symbol("+".to_string())),
        arguments: vector![
            forge::Expression::Integer(Integer::from(1)),
            forge::Expression::Integer(Integer::from(2)),
        ],
    };
    assert_eq!(actual, expected);
}

#[test]
fn parse_nested_array() {
    let tokens = forge::Tokens::from_str("[3.14 [2 3]]");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Array(vector![
        forge::Expression::Float(forge::Float::from_str("3.14")),
        forge::Expression::Array(vector![
            forge::Expression::Integer(Integer::from(2)),
            forge::Expression::Integer(Integer::from(3)),
        ])
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_nested_call() {
    let tokens = forge::Tokens::from_str("(+ 3.14 (- 2 3))");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Call {
        function: Box::new(forge::Expression::Symbol("+".to_string())),
        arguments: vector![
            forge::Expression::Float(forge::Float::from_str("3.14")),
            forge::Expression::Call {
                function: Box::new(forge::Expression::Symbol("-".to_string())),
                arguments: vector![
                    forge::Expression::Integer(Integer::from(2)),
                    forge::Expression::Integer(Integer::from(3)),
                ]
            }
        ],
    };
    assert_eq!(actual, expected);
}

#[test]
fn parse_call_inside_array() {
    let tokens = forge::Tokens::from_str("[3.14 (+ 2 3)]");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Array(vector![
        forge::Expression::Float(forge::Float::from_str("3.14")),
        forge::Expression::Call {
            function: Box::new(forge::Expression::Symbol("+".to_string())),
            arguments: vector![
                forge::Expression::Integer(Integer::from(2)),
                forge::Expression::Integer(Integer::from(3)),
            ]
        }
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_array_inside_call() {
    let tokens = forge::Tokens::from_str("(+ 3.14 [2 3])");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Call {
        function: Box::new(forge::Expression::Symbol("+".to_string())),
        arguments: vector![
            forge::Expression::Float(forge::Float::from_str("3.14")),
            forge::Expression::Array(vector![
                forge::Expression::Integer(Integer::from(2)),
                forge::Expression::Integer(Integer::from(3)),
            ])
        ],
    };
    assert_eq!(actual, expected);
}

#[test]
fn parse_rational() {
    let tokens = forge::Tokens::from_str("1/2");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Ratio(Rational::from((Integer::from(1), Integer::from(2))));
    assert_eq!(actual, expected);
}

#[test]
fn parse_map() {
    let tokens = forge::Tokens::from_str("{:a 1 :b 2}");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Map(hashmap![
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(1)),
        forge::Expression::Keyword(":b".to_string()) => forge::Expression::Integer(Integer::from(2)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_true() {
    let tokens = forge::Tokens::from_str("true");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Bool(true);
    assert_eq!(actual, expected);
}

#[test]
fn parse_false() {
    let tokens = forge::Tokens::from_str("false");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Bool(false);
    assert_eq!(actual, expected);
}

#[test]
fn parse_nil() {
    let tokens = forge::Tokens::from_str("nil");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
}

#[test]
fn parse_quote() {
    let tokens = forge::Tokens::from_str("'(1 2)");
    let actual = forge::parse(tokens);
    let expected = forge::Expression::Quote(Box::new(forge::Expression::Call {
        function: Box::new(forge::Expression::Integer(Integer::from(1))),
        arguments: vector![forge::Expression::Integer(Integer::from(2)),],
    }));
    assert_eq!(actual, expected);
}
