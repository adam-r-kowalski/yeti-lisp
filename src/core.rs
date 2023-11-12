extern crate alloc;

use crate::effect::{error, Effect};
use crate::expression::{Call, Environment, Pattern};
use crate::Expression::{Integer, Module, NativeFunction, Ratio};
use crate::{
    array, evaluate_expressions, extract, html, map, pattern_match, ratio, server, sql, Expression,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use im::{ordmap, vector, Vector};
use rug;

pub fn truthy(expression: &Expression) -> bool {
    match expression {
        Expression::Nil => false,
        Expression::Bool(false) => false,
        _ => true,
    }
}

pub type Result = core::result::Result<(Environment, Expression), Effect>;

pub fn environment() -> Environment {
    ordmap! {
        "=".to_string() => NativeFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            Ok((env, Expression::Bool(args[0] == args[1])))
          }
        ),
        "+".to_string() => NativeFunction(
          |env, args| {
            if args.len() == 0 {
                return Ok((env, Integer(0.into())));
            }
            let (env, args) = evaluate_expressions(env, args)?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                    (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs + rhs).into())),
                    _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
          }
        ),
        "-".to_string() => NativeFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs - rhs).into())),
                  _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
          }
        ),
        "*".to_string() => NativeFunction(
          |env, args| {
            if args.len() == 0 {
                return Ok((env, Integer(1.into())));
            }
            let (env, args) = evaluate_expressions(env, args)?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs * rhs).into())),
                  (Integer(lhs), Ratio(rhs)) => Ok(ratio((lhs * rhs).into())),
                  (Ratio(lhs), Integer(rhs)) => Ok(ratio((lhs * rhs).into())),
                  (lhs, rhs) => Err(error(&format!("Cannot multiply {} and {}", lhs, rhs))),
                }
            })?;
            Ok((env, result))
          }
        ),
        "/".to_string() => NativeFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(ratio(rug::Rational::from((lhs, rhs)))),
                  _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
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
                    let body = args.skip(1);
                    let function = Expression::Function(vector![Pattern { parameters, body }]);
                    Ok((env, function))
                } else {
                    let patterns = args.iter().try_fold(Vector::new(), |mut patterns, pattern| {
                        let call = extract::call(pattern.clone())?;
                        let parameters = extract::array(*call.function)?;
                        let body = call.arguments;
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
            let original_env = env.clone();
            let (bindings, body) = args.split_at(1);
            let bindings = extract::array(bindings[0].clone())?;
            let env = bindings.iter().array_chunks().try_fold(env, |env, [pattern, value]| {
                let (env, value) = crate::evaluate(env, value.clone())?;
                let env = pattern_match(env, pattern.clone(), value)?;
                Ok(env)
            })?;
            let (_, value) = body.iter().try_fold((env, Expression::Nil), |(env, _), expression| {
                crate::evaluate(env, expression.clone())
            })?;
            Ok((original_env, value))
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
        "bound?".to_string() => NativeFunction(
            |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let name = extract::symbol(args[0].clone())?;
                let result = env.contains_key(&name);
                Ok((env, Expression::Bool(result)))
            }
        ),
        "do".to_string() => NativeFunction(
            |env, args| {
                let original_env = env.clone();
                let (_, result) = args.iter().try_fold((env, Expression::Nil), |(env, _), expression| {
                    crate::evaluate(env, expression.clone())
                })?;
                Ok((original_env, result))
            }
        ),
        "when".to_string() => NativeFunction(
            |env, args| {
                let (condition, body) = args.split_at(1);
                let (env, condition) = crate::evaluate(env, condition[0].clone())?;
                if truthy(&condition) {
                    crate::evaluate(env, Expression::Call(Call{
                        function: Box::new(Expression::Symbol("do".to_string())),
                        arguments: body,
                    }))
                } else {
                    Ok((env, Expression::Nil))
                }
            }
        ),
        "import".to_string() => NativeFunction(
            |env, args| {
                let name = extract::symbol(args[0].clone())?;
                let path = format!("{}.yeti", name);
                let (mut env, source) = crate::evaluate(env, Expression::Call(Call{
                    function: Box::new(Expression::NamespacedSymbol(vec![
                        "io".to_string(),
                        "read-file-sync".to_string()
                    ])),
                    arguments: vector![Expression::String(path)],
                }))?;
                let source = extract::string(source)?;
                let tokens = crate::Tokens::from_str(&source);
                let expressions = crate::parse_module(tokens);
                let mut module = environment();
                module.insert("*name*".to_string(), Expression::String(name.clone()));
                module.insert("io".to_string(), env.get("io").unwrap().clone());
                let module = expressions.iter().try_fold(module, |env, expression| {
                    let (env, _) = crate::evaluate(env, expression.clone())?;
                    Ok(env)
                })?;
                env.insert(name, Expression::Module(module));
                Ok((env, Expression::Nil))
            }
        ),
        "assoc".to_string() => NativeFunction(map::assoc),
        "dissoc".to_string() => NativeFunction(map::dissoc),
        "merge".to_string() => NativeFunction(map::merge),
        "get".to_string() => NativeFunction(map::get),
        "nth".to_string() => NativeFunction(array::nth),
        "count".to_string() => NativeFunction(array::count),
        "html".to_string() => Module(html::environment()),
        "server".to_string() => Module(server::environment()),
        "sql".to_string() => Module(sql::environment())
    }
}
