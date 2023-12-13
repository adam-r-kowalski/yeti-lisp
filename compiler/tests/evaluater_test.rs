// use compiler;
// use compiler::expression::Call;
// use im::{ordmap, vector};
// use rug::Integer;
//
// type Result = std::result::Result<(), compiler::effect::Effect>;
//
// #[tokio::test]
// async fn evaluate_keyword() -> Result {
//     let env = compiler::Environment::new();
//     let (_, actual) = compiler::evaluate_source(env, ":x").await?;
//     let expected = compiler::Expression::Keyword(":x".to_string());
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_string() -> Result {
//     let env = compiler::Environment::new();
//     let (_, actual) = compiler::evaluate_source(env, r#""hello""#).await?;
//     let expected = compiler::Expression::String("hello".to_string());
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_integer() -> Result {
//     let env = compiler::Environment::new();
//     let (_, actual) = compiler::evaluate_source(env, "5").await?;
//     let expected = compiler::Expression::Integer(Integer::from(5));
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_float() -> Result {
//     let env = compiler::Environment::new();
//     let (_, actual) = compiler::evaluate_source(env, "3.14").await?;
//     let expected = compiler::Expression::Float(compiler::Float::from_str("3.14"));
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_symbol_bound_to_integer() -> Result {
//     let env = ordmap! {
//         "x".to_string() => compiler::Expression::Integer(Integer::from(5))
//     };
//     let (_, actual) = compiler::evaluate_source(env, "x").await?;
//     let expected = compiler::Expression::Integer(Integer::from(5));
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_symbol_bound_to_function() -> Result {
//     let env = ordmap! {
//         "double".to_string() => compiler::Expression::NativeFunction(
//           |env, args| {
//             Box::pin(async move {
//                 let (env, args) = compiler::evaluate_expressions(env, args).await?;
//                 match &args[0] {
//                   compiler::Expression::Integer(i) => Ok((env, compiler::Expression::Integer(i * Integer::from(2)))),
//                   _ => panic!("Expected integer argument"),
//                 }
//             })
//           }
//         )
//     };
//     let (_, actual) = compiler::evaluate_source(env, "(double 5)").await?;
//     let expected = compiler::Expression::Integer(Integer::from(10));
//     assert_eq!(actual, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn evaluate_quote() -> Result {
//     let env = compiler::Environment::new();
//     let (_, actual) = compiler::evaluate_source(env, "'(1 2)").await?;
//     let expected = compiler::Expression::Call(Call {
//         function: Box::new(compiler::Expression::Integer(Integer::from(1))),
//         arguments: vector![compiler::Expression::Integer(Integer::from(2)),],
//     });
//     assert_eq!(actual, expected);
//     Ok(())
// }
