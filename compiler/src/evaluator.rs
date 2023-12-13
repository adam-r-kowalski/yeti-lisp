// extern crate alloc;
//
// use crate::effect::{error, Effect};
// use crate::expression::{Call, Environment, Pattern, Result};
// use crate::extract;
// use crate::Expression;
// use alloc::boxed::Box;
// use alloc::format;
// use alloc::string::{String, ToString};
// use alloc::vec;
// use alloc::vec::Vec;
// use async_recursion::async_recursion;
// use im::Vector;
//
// fn evaluate_symbol(environment: Environment, symbol: String) -> Result {
//     if let Some(e) = environment.get(&symbol) {
//         Ok((environment.clone(), e.clone()))
//     } else {
//         Err(error(&format!(
//             "Symbol {} not found in environment",
//             symbol
//         )))
//     }
// }
//
// fn evaluate_namespaced_symbol(environment: Environment, symbol: &[String]) -> Result {
//     let (first, rest) = symbol.split_first().unwrap();
//     let (environment, value) = evaluate_symbol(environment, first.clone())?;
//     if rest.is_empty() {
//         Ok((environment, value))
//     } else {
//         let module = extract::module(value)?;
//         let (_, value) = evaluate_namespaced_symbol(module, rest)?;
//         Ok((environment, value))
//     }
// }
//
// pub fn pattern_match(
//     env: Environment,
//     pattern: Expression,
//     value: Expression,
// ) -> core::result::Result<Environment, Effect> {
//     match pattern {
//         Expression::Symbol(s) => {
//             let mut env = env.clone();
//             env.insert(s, value);
//             Ok(env)
//         }
//         Expression::Keyword(k1) => match value {
//             Expression::Keyword(k2) if k1 == k2 => Ok(env),
//             _ => Err(error(&format!(
//                 "Cannot pattern match {} with {}",
//                 Expression::Keyword(k1),
//                 value
//             ))),
//         },
//         Expression::String(s1) => match value {
//             Expression::String(s2) if s1 == s2 => Ok(env),
//             _ => Err(error(&format!(
//                 "Cannot pattern match {} with {}",
//                 Expression::String(s1),
//                 value
//             ))),
//         },
//         Expression::Integer(i1) => match value {
//             Expression::Integer(i2) if i1 == i2 => Ok(env),
//             _ => Err(error(&format!(
//                 "Cannot pattern match {} with {}",
//                 Expression::Integer(i1),
//                 value
//             ))),
//         },
//         Expression::Nil => match value {
//             Expression::Nil => Ok(env),
//             _ => Err(error(&format!(
//                 "Cannot pattern match {} with {}",
//                 Expression::Nil,
//                 value
//             ))),
//         },
//         Expression::Array(patterns) => {
//             if let Expression::Array(values) = value {
//                 let env = patterns
//                     .into_iter()
//                     .zip(values.into_iter())
//                     .try_fold(env, |env, (pattern, value)| {
//                         pattern_match(env, pattern, value)
//                     })?;
//                 Ok(env)
//             } else {
//                 Err(error(&format!(
//                     "Cannot pattern match {} with {}",
//                     Expression::Array(patterns),
//                     value
//                 )))
//             }
//         }
//         Expression::Map(map) => {
//             if let Expression::Map(m) = value {
//                 let env = map.into_iter().try_fold(env, |env, (pattern, value)| {
//                     if let Some(v) = m.get(&pattern) {
//                         pattern_match(env, value, v.clone())
//                     } else {
//                         Err(error(&format!(
//                             "Cannot pattern match {} with {}",
//                             pattern, value
//                         )))
//                     }
//                 })?;
//                 Ok(env)
//             } else {
//                 Err(error(&format!(
//                     "Cannot pattern match {} with {}",
//                     Expression::Map(map),
//                     value
//                 )))
//             }
//         }
//         _ => Err(error(&format!(
//             "Cannot pattern match {} with {}",
//             pattern, value
//         ))),
//     }
// }
//
// fn find_pattern_match(
//     env: Environment,
//     patterns: Vector<Pattern>,
//     arguments: Vector<Expression>,
// ) -> core::result::Result<(Environment, Vector<Expression>), Effect> {
//     let mut failures = vec![];
//     for Pattern { parameters, body } in patterns {
//         let result = pattern_match(
//             env.clone(),
//             Expression::Array(parameters.clone()),
//             Expression::Array(arguments.clone()),
//         );
//         match result {
//             Ok(env) => return Ok((env, body)),
//             Err(e) => failures.push(e),
//         };
//     }
//     let error_message = failures.iter().fold(String::new(), |mut s, e| {
//         s.push_str(&format!("{}\n", e));
//         s
//     });
//     Err(error(&error_message))
// }
//
// async fn evaluate_call(environment: Environment, call: Call) -> Result {
//     let Call {
//         function,
//         arguments,
//     } = call;
//     let (environment, function) = evaluate(environment.clone(), *function).await?;
//     match function {
//         Expression::Function(function) => {
//             let original_environment = environment.clone();
//             let (_, arguments) = evaluate_expressions(environment, arguments).await?;
//             let cloned_function = function.clone();
//             let (mut env, body) = find_pattern_match(function.env, function.patterns, arguments)?;
//             env.insert(
//                 "recur".to_string(),
//                 Expression::Function(cloned_function.clone()),
//             );
//             if let Some(Expression::Symbol(name)) = env.get("*self*") {
//                 env.insert(name.to_string(), Expression::Function(cloned_function));
//             }
//             let (_, results) = evaluate_expressions(env, body).await?;
//             Ok((original_environment, results.last().unwrap_or(&Expression::Nil).clone()))
//         }
//         Expression::NativeFunction(f) => {
//             let (env, value) = f(environment, arguments).await?;
//             Ok((env, value))
//         }
//         Expression::Keyword(k) => {
//             let (environment, arguments) = evaluate_expressions(environment, arguments).await?;
//             match &arguments[0] {
//                 Expression::Map(m) => {
//                     if let Some(v) = m.get(&Expression::Keyword(k)) {
//                         Ok((environment, v.clone()))
//                     } else if arguments.len() == 2 {
//                         Ok((environment, arguments[1].clone()))
//                     } else {
//                         Ok((environment, Expression::Nil))
//                     }
//                 }
//                 e => Err(error(&format!("Cannot call keyword {} on {}", k, e))),
//             }
//         }
//         Expression::Map(m) => {
//             let (environment, arguments) = evaluate_expressions(environment, arguments).await?;
//             if let Some(v) = m.get(&arguments[0]) {
//                 Ok((environment, v.clone()))
//             } else if arguments.len() == 2 {
//                 Ok((environment, arguments[1].clone()))
//             } else {
//                 Ok((environment, Expression::Nil))
//             }
//         }
//         _ => Err(error(&format!("Cannot call {}", function))),
//     }
// }
//
// async fn evaluate_deref(environment: Environment, expression: Expression) -> Result {
//     let (environment, expression) = evaluate(environment, expression).await?;
//     let atom = extract::atom(expression)?;
//     let value = atom.0.lock().await;
//     Ok((environment, value.clone()))
// }
//
// #[async_recursion]
// pub async fn evaluate(environment: Environment, expression: Expression) -> Result {
//     match expression {
//         Expression::Symbol(s) => evaluate_symbol(environment, s),
//         Expression::NamespacedSymbol(s) => evaluate_namespaced_symbol(environment, &s),
//         Expression::Call(call) => evaluate_call(environment, call).await,
//         Expression::Array(a) => {
//             let (environment, a) = evaluate_expressions(environment, a).await?;
//             Ok((environment, Expression::Array(a)))
//         }
//         Expression::Map(m) => {
//             let mut environment = environment.clone();
//             let mut new_map = im::OrdMap::new();
//             for (k, v) in m {
//                 let (e, k) = evaluate(environment, k).await?;
//                 let (e, v) = evaluate(e, v).await?;
//                 new_map.insert(k, v);
//                 environment = e;
//             }
//             Ok((environment, Expression::Map(new_map)))
//         }
//         Expression::Quote(e) => Ok((environment, *e)),
//         Expression::Deref(e) => evaluate_deref(environment, *e).await,
//         e => Ok((environment, e)),
//     }
// }
//
// pub async fn evaluate_expressions(
//     env: Environment,
//     exprs: Vector<Expression>,
// ) -> core::result::Result<(Environment, Vector<Expression>), Effect> {
//     let futures: Vec<_> = exprs
//         .iter()
//         .map(|expr| {
//             let env = env.clone();
//             let expr = expr.clone();
//             async move {
//                 let (_, value) = evaluate(env, expr).await?;
//                 Ok(value)
//             }
//         })
//         .collect();
//     let results = futures::future::join_all(futures).await;
//     let mut values = Vector::new();
//     for result in results {
//         match result {
//             Ok(v) => values.push_back(v),
//             Err(e) => return Err(e),
//         }
//     }
//     Ok((env, values))
// }
//
// pub async fn evaluate_source(
//     env: Environment,
//     source: &str,
// ) -> core::result::Result<(Environment, Expression), Effect> {
//     let tokens = crate::Tokens::from_str(source);
//     let expression = crate::parse(tokens);
//     evaluate(env, expression).await
// }
