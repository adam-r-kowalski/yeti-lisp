use im::{hashmap, vector};
use rug::{Integer, Rational};
use yeti;
use yeti::expression::Call;

#[test]
fn parse_symbol() {
    let tokens = yeti::Tokens::from_str("x");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Symbol("x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_keyword() {
    let tokens = yeti::Tokens::from_str(":x");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_string() {
    let tokens = yeti::Tokens::from_str(r#""hello""#);
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn parse_integer() {
    let tokens = yeti::Tokens::from_str("123");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Integer(Integer::from(123));
    assert_eq!(actual, expected);
}

#[test]
fn parse_float() {
    let tokens = yeti::Tokens::from_str("3.14");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Float(yeti::Float::from_str("3.14"));
    assert_eq!(actual, expected);
}

#[test]
fn parse_homogenous_array() {
    let tokens = yeti::Tokens::from_str("[1 2 3]");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Array(vector![
        yeti::Expression::Integer(Integer::from(1)),
        yeti::Expression::Integer(Integer::from(2)),
        yeti::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_heterogenous_array() {
    let tokens = yeti::Tokens::from_str("[3.14 2 3]");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Array(vector![
        yeti::Expression::Float(yeti::Float::from_str("3.14")),
        yeti::Expression::Integer(Integer::from(2)),
        yeti::Expression::Integer(Integer::from(3)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_call() {
    let tokens = yeti::Tokens::from_str("(+ 1 2)");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Call(Call {
        function: Box::new(yeti::Expression::Symbol("+".to_string())),
        arguments: vector![
            yeti::Expression::Integer(Integer::from(1)),
            yeti::Expression::Integer(Integer::from(2)),
        ],
    });
    assert_eq!(actual, expected);
}

#[test]
fn parse_nested_array() {
    let tokens = yeti::Tokens::from_str("[3.14 [2 3]]");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Array(vector![
        yeti::Expression::Float(yeti::Float::from_str("3.14")),
        yeti::Expression::Array(vector![
            yeti::Expression::Integer(Integer::from(2)),
            yeti::Expression::Integer(Integer::from(3)),
        ])
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_nested_call() {
    let tokens = yeti::Tokens::from_str("(+ 3.14 (- 2 3))");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Call(Call {
        function: Box::new(yeti::Expression::Symbol("+".to_string())),
        arguments: vector![
            yeti::Expression::Float(yeti::Float::from_str("3.14")),
            yeti::Expression::Call(Call {
                function: Box::new(yeti::Expression::Symbol("-".to_string())),
                arguments: vector![
                    yeti::Expression::Integer(Integer::from(2)),
                    yeti::Expression::Integer(Integer::from(3)),
                ]
            })
        ],
    });
    assert_eq!(actual, expected);
}

#[test]
fn parse_call_inside_array() {
    let tokens = yeti::Tokens::from_str("[3.14 (+ 2 3)]");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Array(vector![
        yeti::Expression::Float(yeti::Float::from_str("3.14")),
        yeti::Expression::Call(Call {
            function: Box::new(yeti::Expression::Symbol("+".to_string())),
            arguments: vector![
                yeti::Expression::Integer(Integer::from(2)),
                yeti::Expression::Integer(Integer::from(3)),
            ]
        })
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_array_inside_call() {
    let tokens = yeti::Tokens::from_str("(+ 3.14 [2 3])");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Call(Call {
        function: Box::new(yeti::Expression::Symbol("+".to_string())),
        arguments: vector![
            yeti::Expression::Float(yeti::Float::from_str("3.14")),
            yeti::Expression::Array(vector![
                yeti::Expression::Integer(Integer::from(2)),
                yeti::Expression::Integer(Integer::from(3)),
            ])
        ],
    });
    assert_eq!(actual, expected);
}

#[test]
fn parse_rational() {
    let tokens = yeti::Tokens::from_str("1/2");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Ratio(Rational::from((Integer::from(1), Integer::from(2))));
    assert_eq!(actual, expected);
}

#[test]
fn parse_map() {
    let tokens = yeti::Tokens::from_str("{:a 1 :b 2}");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Map(hashmap![
        yeti::Expression::Keyword(":a".to_string()) => yeti::Expression::Integer(Integer::from(1)),
        yeti::Expression::Keyword(":b".to_string()) => yeti::Expression::Integer(Integer::from(2)),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn parse_true() {
    let tokens = yeti::Tokens::from_str("true");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Bool(true);
    assert_eq!(actual, expected);
}

#[test]
fn parse_false() {
    let tokens = yeti::Tokens::from_str("false");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Bool(false);
    assert_eq!(actual, expected);
}

#[test]
fn parse_nil() {
    let tokens = yeti::Tokens::from_str("nil");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
}

#[test]
fn parse_quote() {
    let tokens = yeti::Tokens::from_str("'(1 2)");
    let actual = yeti::parse(tokens);
    let expected = yeti::Expression::Quote(Box::new(yeti::Expression::Call(Call {
        function: Box::new(yeti::Expression::Integer(Integer::from(1))),
        arguments: vector![yeti::Expression::Integer(Integer::from(2)),],
    })));
    assert_eq!(actual, expected);
}
