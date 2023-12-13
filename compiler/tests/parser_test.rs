// use compiler;
// use compiler::expression::Call;
// use im::{ordmap, vector};
// use rug::{Integer, Rational};
//
// #[test]
// fn parse_symbol() {
//     let tokens = compiler::Tokens::from_str("x");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Symbol("x".to_string());
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_keyword() {
//     let tokens = compiler::Tokens::from_str(":x");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Keyword(":x".to_string());
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_string() {
//     let tokens = compiler::Tokens::from_str(r#""hello""#);
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::String("hello".to_string());
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_integer() {
//     let tokens = compiler::Tokens::from_str("123");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Integer(Integer::from(123));
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_float() {
//     let tokens = compiler::Tokens::from_str("3.14");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Float(compiler::Float::from_str("3.14"));
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_homogenous_array() {
//     let tokens = compiler::Tokens::from_str("[1 2 3]");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Array(vector![
//         compiler::Expression::Integer(Integer::from(1)),
//         compiler::Expression::Integer(Integer::from(2)),
//         compiler::Expression::Integer(Integer::from(3)),
//     ]);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_heterogenous_array() {
//     let tokens = compiler::Tokens::from_str("[3.14 2 3]");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Array(vector![
//         compiler::Expression::Float(compiler::Float::from_str("3.14")),
//         compiler::Expression::Integer(Integer::from(2)),
//         compiler::Expression::Integer(Integer::from(3)),
//     ]);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_call() {
//     let tokens = compiler::Tokens::from_str("(+ 1 2)");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Call(Call {
//         function: Box::new(compiler::Expression::Symbol("+".to_string())),
//         arguments: vector![
//             compiler::Expression::Integer(Integer::from(1)),
//             compiler::Expression::Integer(Integer::from(2)),
//         ],
//     });
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_nested_array() {
//     let tokens = compiler::Tokens::from_str("[3.14 [2 3]]");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Array(vector![
//         compiler::Expression::Float(compiler::Float::from_str("3.14")),
//         compiler::Expression::Array(vector![
//             compiler::Expression::Integer(Integer::from(2)),
//             compiler::Expression::Integer(Integer::from(3)),
//         ])
//     ]);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_nested_call() {
//     let tokens = compiler::Tokens::from_str("(+ 3.14 (- 2 3))");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Call(Call {
//         function: Box::new(compiler::Expression::Symbol("+".to_string())),
//         arguments: vector![
//             compiler::Expression::Float(compiler::Float::from_str("3.14")),
//             compiler::Expression::Call(Call {
//                 function: Box::new(compiler::Expression::Symbol("-".to_string())),
//                 arguments: vector![
//                     compiler::Expression::Integer(Integer::from(2)),
//                     compiler::Expression::Integer(Integer::from(3)),
//                 ]
//             })
//         ],
//     });
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_call_inside_array() {
//     let tokens = compiler::Tokens::from_str("[3.14 (+ 2 3)]");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Array(vector![
//         compiler::Expression::Float(compiler::Float::from_str("3.14")),
//         compiler::Expression::Call(Call {
//             function: Box::new(compiler::Expression::Symbol("+".to_string())),
//             arguments: vector![
//                 compiler::Expression::Integer(Integer::from(2)),
//                 compiler::Expression::Integer(Integer::from(3)),
//             ]
//         })
//     ]);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_array_inside_call() {
//     let tokens = compiler::Tokens::from_str("(+ 3.14 [2 3])");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Call(Call {
//         function: Box::new(compiler::Expression::Symbol("+".to_string())),
//         arguments: vector![
//             compiler::Expression::Float(compiler::Float::from_str("3.14")),
//             compiler::Expression::Array(vector![
//                 compiler::Expression::Integer(Integer::from(2)),
//                 compiler::Expression::Integer(Integer::from(3)),
//             ])
//         ],
//     });
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_rational() {
//     let tokens = compiler::Tokens::from_str("1/2");
//     let actual = compiler::parse(tokens);
//     let expected =
//         compiler::Expression::Ratio(Rational::from((Integer::from(1), Integer::from(2))));
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_map() {
//     let tokens = compiler::Tokens::from_str("{:a 1 :b 2}");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Map(ordmap![
//         compiler::Expression::Keyword(":a".to_string()) => compiler::Expression::Integer(Integer::from(1)),
//         compiler::Expression::Keyword(":b".to_string()) => compiler::Expression::Integer(Integer::from(2))
//     ]);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_true() {
//     let tokens = compiler::Tokens::from_str("true");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Bool(true);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_false() {
//     let tokens = compiler::Tokens::from_str("false");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Bool(false);
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_nil() {
//     let tokens = compiler::Tokens::from_str("nil");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Nil;
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_quote() {
//     let tokens = compiler::Tokens::from_str("'(1 2)");
//     let actual = compiler::parse(tokens);
//     let expected = compiler::Expression::Quote(Box::new(compiler::Expression::Call(Call {
//         function: Box::new(compiler::Expression::Integer(Integer::from(1))),
//         arguments: vector![compiler::Expression::Integer(Integer::from(2)),],
//     })));
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn parse_deref() {
//     let tokens = compiler::Tokens::from_str("@x");
//     let actual = compiler::parse(tokens);
//     let expected =
//         compiler::Expression::Deref(Box::new(compiler::Expression::Symbol("x".to_string())));
//     assert_eq!(actual, expected);
// }
