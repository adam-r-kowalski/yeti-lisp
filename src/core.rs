extern crate alloc;

use crate::effect::error;
use crate::expression::{Call, Environment, Pattern};
use crate::Expression::{Integer, NativeFunction, Ratio};
use crate::{
    array, evaluate_expressions, extract, html, map, pattern_match, ratio, server, sql, Expression,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use im::{ordmap, vector, OrdMap, Vector};
use rug;

pub fn truthy(expression: &Expression) -> bool {
    match expression {
        Expression::Nil => false,
        Expression::Bool(false) => false,
        _ => true,
    }
}

pub fn environment() -> Environment {
    Environment {
        bindings: ordmap! {
            "=".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                Ok((env, Expression::Bool(args[0] == args[1])))
              }
            ),
            "+".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                match (&args[0], &args[1]) {
                  (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs + rhs).into()))),
                  _ => Err(error("Expected integer argument")),
                }
              }
            ),
            "-".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                match (&args[0], &args[1]) {
                  (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs - rhs).into()))),
                  _ => Err(error("Expected integer argument")),
                }
              }
            ),
            "*".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                match (&args[0], &args[1]) {
                  (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs * rhs).into()))),
                  (Integer(lhs), Ratio(rhs)) => Ok((env, ratio((lhs * rhs).into()))),
                  (Ratio(lhs), Integer(rhs)) => Ok((env, ratio((lhs * rhs).into()))),
                  (lhs, rhs) => Err(error(&format!("Cannot multiply {} and {}", lhs, rhs))),
                }
              }
            ),
            "/".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                match (&args[0], &args[1]) {
                  (Integer(lhs), Integer(rhs)) => Ok((env, ratio(rug::Rational::from((lhs, rhs))))),
                  _ => Err(error("Expected integer argument")),
                }
              }
            ),
            "if".to_string() => NativeFunction(
              |env, args| {
                let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
                let (env, condition) = crate::evaluate(env, condition)?;
                crate::evaluate(env, if truthy(&condition) { then } else { otherwise })
              }
            ),
            "def".to_string() => NativeFunction(
              |env, args| {
                let (name, value) = (args[0].clone(), args[1].clone());
                let (env, value) = crate::evaluate(env, value)?;
                let name = extract::symbol(name)?;
                let mut new_env = env.clone();
                new_env.insert(name, value);
                Ok((new_env, Expression::Nil))
              }
            ),
            "fn".to_string() => NativeFunction(
                |env, args| {
                    if let Expression::Array(array) = &args[0] {
                        let parameters = array.clone();
                        let body = args[1].clone();
                        let function = Expression::Function(vector![Pattern { parameters, body }]);
                        Ok((env, function))
                    } else {
                        let patterns = args.iter().try_fold(Vector::new(), |mut patterns, pattern| {
                            let call = extract::call(pattern.clone())?;
                            let parameters = extract::array(*call.function)?;
                            let body = call.arguments[0].clone();
                            patterns.push_back(Pattern { parameters, body });
                            Ok(patterns)
                        })?;
                        let function = Expression::Function(patterns);
                        Ok((env, function))
                    }
                }
            ),

            "defn".to_string() => NativeFunction(
              |env, args| {
                let (env, function) = crate::evaluate(env, Expression::Call(Call{
                    function: Box::new(Expression::Symbol("fn".to_string())),
                    arguments: args.iter().skip(1).cloned().collect(),
                }))?;
                crate::evaluate(env, Expression::Call(Call{
                    function: Box::new(Expression::Symbol("def".to_string())),
                    arguments: vector![args[0].clone(), function],
                }))
              }
            ),
            "eval".to_string() => NativeFunction(
              |env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                crate::evaluate(env, arg)
              }
            ),
            "read-string".to_string() => NativeFunction(
              |env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                let s = extract::string(arg)?;
                let tokens = crate::Tokens::from_str(&s);
                let expression = crate::parse(tokens);
                Ok((env, expression))
              }
            ),
            "assert".to_string() => NativeFunction(
              |env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                if truthy(&arg) {
                    Ok((env, Expression::Nil))
                } else {
                    Err(error("Assertion failed"))
                }
              }
            ),
            "let".to_string() => NativeFunction(
              |env, args| {
                let (bindings, body) = (args[0].clone(), args[1].clone());
                let bindings = extract::array(bindings)?;
                let env = bindings.iter().array_chunks().try_fold(env, |env, [pattern, value]| {
                    let (env, value) = crate::evaluate(env, value.clone())?;
                    let env = pattern_match(env, pattern.clone(), value)?;
                    Ok(env)
                })?;
                crate::evaluate(env, body)
              }
            ),
            "for".to_string() => NativeFunction(
              |env, args| {
                let (bindings, body) = (args[0].clone(), args[1].clone());
                let bindings = extract::array(bindings)?;
                let pattern = bindings[0].clone();
                let (env, values) = crate::evaluate(env, bindings[1].clone())?;
                let values = extract::array(values)?;
                let result = values.iter().try_fold(Vector::new(), |mut result, value| {
                    let env = pattern_match(env.clone(), pattern.clone(), value.clone())?;
                    let (_, value) = crate::evaluate(env, body.clone())?;
                    result.push_back(value);
                    Ok(result)
                })?;
                crate::evaluate(env, Expression::Array(result))
              }
            ),
            "str".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let result = args.iter().fold(String::new(), |mut result, arg| {
                    result.push_str(&format!("{}", arg));
                    result
                });
                Ok((env, Expression::String(result)))
              }
            ),
            "assoc".to_string() => NativeFunction(map::assoc),
            "dissoc".to_string() => NativeFunction(map::dissoc),
            "merge".to_string() => NativeFunction(map::merge),
            "get".to_string() => NativeFunction(map::get),
            "nth".to_string() => NativeFunction(array::nth),
            "count".to_string() => NativeFunction(array::count),
            "html/string".to_string() => NativeFunction(html::string),
            "server/start".to_string() => NativeFunction(server::start),
            "server/stop".to_string() => NativeFunction(server::shutdown),
            "sql/connect".to_string() => NativeFunction(sql::connect),
            "sql/string".to_string() => NativeFunction(sql::string),
            "sql/query".to_string() => NativeFunction(sql::query),
            "sql/execute!".to_string() => NativeFunction(sql::execute),
            "sql/tables".to_string() => NativeFunction(sql::tables)
        },
        servers: alloc::sync::Arc::new(spin::Mutex::new(OrdMap::new())),
    }
}
