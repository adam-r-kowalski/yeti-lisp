extern crate alloc;

use crate::effect::error;
use crate::expression::Environment;
use crate::Expression::{Integer, NativeFunction, Ratio};
use crate::{array, evaluate_expressions, extract, html, map, ratio, server, sql, Expression};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use im::{hashmap, vector, HashMap};
use rug;

pub fn environment() -> Environment {
    Environment {
        bindings: hashmap! {
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
                match condition {
                    Expression::Nil | Expression::Bool(false) => crate::evaluate(env, otherwise),
                    _ => crate::evaluate(env, then),
                }
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
                let (parameters, body) = (args[0].clone(), args[1].clone());
                let parameters = extract::array(parameters)?;
                for parameter in parameters.iter() {
                    match parameter {
                        Expression::Symbol(_) => {},
                        _ => return Err(error("Expected symbol")),
                    }
                }
                let body = Box::new(body);
                let function = Expression::Function{parameters, body};
                Ok((env, function))
              }
            ),
            "defn".to_string() => NativeFunction(
              |env, args| {
                let (name, parameters, body) = (args[0].clone(), args[1].clone(), args[2].clone());
                let (env, function) = crate::evaluate(env, Expression::Call{
                    function: Box::new(Expression::Symbol("fn".to_string())),
                    arguments: vector![parameters, body],
                })?;
                crate::evaluate(env, Expression::Call{
                    function: Box::new(Expression::Symbol("def".to_string())),
                    arguments: vector![name, function],
                })
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
            "sql/tables".to_string() => NativeFunction(sql::tables),
        },
        servers: alloc::sync::Arc::new(spin::Mutex::new(HashMap::new())),
    }
}
