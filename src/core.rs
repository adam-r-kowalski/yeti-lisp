extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate_expressions;
use crate::expression::{Environment, Sqlite};
use crate::extract;
use crate::html;
use crate::server;
use crate::Expression;
use crate::Expression::{Integer, NativeFunction, Ratio};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use im::{hashmap, vector, HashMap, Vector};
use rug;
use rusqlite::Connection;

type Result<T> = core::result::Result<T, Effect>;

fn nth(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let arr = extract::array(args[0].clone())?;
    let idx = extract::integer(args[1].clone())?
        .to_usize()
        .ok_or_else(|| error("Index out of range"))?;
    if let Some(value) = arr.get(idx) {
        Ok((env, value.clone()))
    } else if args.len() == 3 {
        Ok((env, args[2].clone()))
    } else {
        Err(error("Index out of range"))
    }
}

fn map_get(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let array = &args[0];
    let key = &args[1];
    let map = extract::map(array.clone())?;
    if let Some(value) = map.get(key) {
        Ok((env, value.clone()))
    } else if args.len() == 3 {
        Ok((env, args[2].clone()))
    } else {
        Ok((env, Expression::Nil))
    }
}

fn sql(expr: Expression) -> Result<Expression> {
    let map = extract::map(expr)?;
    let table_name = extract::keyword(extract::key(map.clone(), ":create-table")?)?;
    let table_name = &table_name[1..];
    let string = format!("CREATE TABLE {} (", table_name).to_string();
    let columns = extract::array(extract::key(map, ":with-columns")?)?;
    let mut string = columns
        .iter()
        .enumerate()
        .try_fold(string, |mut string, (i, column)| {
            let column = extract::array(column.clone())?;
            let name = extract::keyword(column[0].clone())?;
            let name = &name[1..];
            if i > 0 {
                string.push_str(", ");
            }
            string.push_str(name);
            match column[1].clone() {
                Expression::Keyword(type_name) => {
                    let type_name = &type_name[1..].to_uppercase();
                    string.push(' ');
                    string.push_str(type_name);
                }
                Expression::Array(a) => {
                    let type_name = extract::keyword(a[0].clone())?;
                    let type_name = &type_name[1..].to_uppercase();
                    let argument = extract::integer(a[1].clone())?;
                    string.push(' ');
                    string.push_str(type_name);
                    string.push('(');
                    string.push_str(&argument.to_string());
                    string.push(')');
                }
                _ => return Err(error("Expected keyword")),
            };
            column
                .iter()
                .skip(2)
                .try_fold(string, |mut string, expr| match expr {
                    Expression::Keyword(attribute) => {
                        let attribute = &attribute[1..].to_uppercase();
                        string.push(' ');
                        string.push_str(attribute);
                        Ok(string)
                    }
                    Expression::Array(a) => {
                        let attribute = extract::keyword(a[0].clone())?;
                        let attribute = &attribute[1..].to_uppercase();
                        string.push(' ');
                        string.push_str(attribute);
                        match a[1] {
                            Expression::Nil => {
                                string.push_str(" NULL");
                                Ok(string)
                            }
                            _ => Err(error("Expected nil")),
                        }
                    }
                    _ => Err(error("Expected keyword")),
                })
        })?;
    string.push(')');
    Ok(Expression::Array(vector![Expression::String(string)]))
}

pub fn environment() -> Environment {
    Environment {
        bindings: hashmap! {
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
                  _ => Err(error("Expected integer argument")),
                }
              }
            ),
            "/".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                match (&args[0], &args[1]) {
                  (Integer(lhs), Integer(rhs)) => {
                    let rational = rug::Rational::from((lhs, rhs));
                    if rational.is_integer() {
                        Ok((env, Integer(rational.numer().clone())))
                    } else {
                        Ok((env, Ratio(rational)))
                    }
                  },
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
            "assoc".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let (map, key, value) = (args[0].clone(), args[1].clone(), args[2].clone());
                let mut m = extract::map(map)?;
                m.insert(key, value);
                Ok((env, Expression::Map(m)))
              }
            ),
            "dissoc".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let (map, key) = (args[0].clone(), args[1].clone());
                let mut m = extract::map(map)?;
                m.remove(&key);
                Ok((env, Expression::Map(m)))
              }
            ),
            "merge".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let (map1, map2) = (args[0].clone(), args[1].clone());
                let mut map1 = extract::map(map1)?;
                let map2 = extract::map(map2)?;
                map1.extend(map2);
                Ok((env, Expression::Map(map1)))
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
            "html".to_string() => NativeFunction(html),
            "server".to_string() => NativeFunction(server::start),
            "shutdown".to_string() => NativeFunction(server::shutdown),
            "sqlite".to_string() => NativeFunction(|env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                let path = extract::string(arg)?;
                if path == ":memory:" {
                    match Connection::open_in_memory() {
                        Ok(db) => Ok((env, Expression::Sqlite(Sqlite::new(db)))),
                        Err(_) => Err(error("Failed to open SQLite database")),
                    }
                } else {
                    Err(error("Only :memory: is supported"))
                }
            }),
            "sql".to_string() => NativeFunction(|env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let expr = sql(args[0].clone())?;
                Ok((env, expr))
            }),
            "nth".to_string() => NativeFunction(nth),
            "get".to_string() => NativeFunction(map_get),
        },
        servers: alloc::sync::Arc::new(spin::Mutex::new(HashMap::new())),
    }
}
