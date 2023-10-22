extern crate alloc;

use core::net::IpAddr;
use core::net::Ipv4Addr;
use core::net::SocketAddr;

use crate::evaluate_expressions;
use crate::expression::{Environment, Sqlite};
use crate::Expression;
use crate::Expression::{Integer, NativeFunction, Ratio};
use crate::RaisedEffect;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use im::{hashmap, vector, HashMap, Vector};
use rug;
use rusqlite::Connection;
use tokio::sync::broadcast;

pub fn error(message: &str) -> RaisedEffect {
    RaisedEffect {
        environment: environment(),
        effect: "error".to_string(),
        arguments: vector![Expression::String(message.to_string())],
    }
}

type Result<T> = core::result::Result<T, RaisedEffect>;

fn extract_map(expr: Expression) -> Result<HashMap<Expression, Expression>> {
    match expr {
        Expression::Map(m) => Ok(m),
        _ => Err(error("Expected map")),
    }
}

fn extract_key(map: HashMap<Expression, Expression>, key: &str) -> Result<Expression> {
    match map.get(&Expression::Keyword(key.to_string())) {
        Some(expr) => Ok(expr.clone()),
        None => Err(error(&format!("Expected keyword {}", key))),
    }
}

fn extract_string(expr: Expression) -> Result<String> {
    match expr {
        Expression::String(s) => Ok(s),
        _ => Err(error("Expected string")),
    }
}

fn extract_keyword(expr: Expression) -> Result<String> {
    match expr {
        Expression::Keyword(k) => Ok(k),
        _ => Err(error("Expected keyword")),
    }
}

fn extract_symbol(expr: Expression) -> Result<String> {
    match expr {
        Expression::Symbol(s) => Ok(s),
        _ => Err(error("Expected keyword")),
    }
}

fn extract_array(expr: Expression) -> Result<Vector<Expression>> {
    match expr {
        Expression::Array(a) => Ok(a),
        _ => Err(error("Expected array")),
    }
}

fn extract_integer(expr: Expression) -> Result<rug::Integer> {
    match expr {
        Expression::Integer(i) => Ok(i),
        _ => Err(error("Expected integer")),
    }
}

fn nth(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let array = &args[0];
    let index = &args[1];
    let arr = extract_array(array.clone())?;
    let idx = extract_integer(index.clone())?
        .to_usize()
        .ok_or_else(|| error("Index out of range"))?;

    match args.len() {
        2 => {
            let result = arr
                .get(idx)
                .cloned()
                .ok_or_else(|| error("Index out of range"))?;
            Ok((env, result))
        }
        3 => {
            let default_value = &args[2];
            let result = arr
                .get(idx)
                .cloned()
                .unwrap_or_else(|| default_value.clone());
            Ok((env, result))
        }
        _ => Err(error("Invalid number of arguments")),
    }
}

fn self_closing(tag: &str) -> bool {
    match tag {
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn style_tag(style_map: HashMap<Expression, Expression>, string: &mut String) -> Result<()> {
    string.push_str("<style>");
    for (k, v) in style_map {
        let selector = extract_keyword(k.clone())?;
        let rules_map = extract_map(v.clone())?;
        string.push_str(&selector[1..]);
        string.push_str(" { ");
        for (rule_key, rule_value) in rules_map {
            let rule_property = extract_keyword(rule_key.clone())?;
            let rule_val_str = extract_string(rule_value)?;
            string.push_str(&rule_property[1..]);
            string.push_str(": ");
            string.push_str(&rule_val_str);
            string.push_str("; ");
        }
        string.push_str("}");
    }
    string.push_str("</style>");
    Ok(())
}

fn html(expr: Expression, string: &mut String) -> Result<()> {
    match expr {
        Expression::Array(a) => {
            let keyword = extract_keyword(a[0].clone())?;
            let keyword = &keyword[1..];
            if keyword == "style" {
                let style_map = extract_map(a[1].clone())?;
                return style_tag(style_map, string);
            }
            string.push('<');
            string.push_str(keyword);
            if a.len() > 1 {
                if let Expression::Map(m) = &a[1] {
                    let mut entries = Vec::new();
                    for (k, v) in m.iter() {
                        let k = extract_keyword(k.clone())?;
                        entries.push((k, v.clone()));
                    }
                    entries.sort_by_key(|entry| entry.0.clone());
                    for (k, v) in entries {
                        string.push(' ');
                        string.push_str(&k[1..]);
                        string.push_str("=\"");
                        let s = extract_string(v)?;
                        string.push_str(&s);
                        string.push('"');
                    }
                    if self_closing(keyword) {
                        string.push_str(" />");
                        Ok(())
                    } else {
                        string.push('>');
                        for expr in a.iter().skip(2) {
                            html(expr.clone(), string)?;
                        }
                        string.push_str("</");
                        string.push_str(keyword);
                        string.push('>');
                        Ok(())
                    }
                } else if self_closing(keyword) {
                    string.push_str(" />");
                    Ok(())
                } else {
                    string.push('>');
                    for expr in a.iter().skip(1) {
                        html(expr.clone(), string)?;
                    }
                    string.push_str("</");
                    string.push_str(keyword);
                    string.push('>');
                    Ok(())
                }
            } else if self_closing(keyword) {
                string.push_str(" />");
                Ok(())
            } else {
                string.push_str("></");
                string.push_str(keyword);
                string.push('>');
                Ok(())
            }
        }
        Expression::String(s) => {
            string.push_str(&s);
            Ok(())
        }
        _ => Err(error("Expected keyword")),
    }
}

fn sql(expr: Expression) -> Result<Expression> {
    let map = extract_map(expr)?;
    let table_name = extract_keyword(extract_key(map.clone(), ":create-table")?)?;
    let table_name = &table_name[1..];
    let string = format!("CREATE TABLE {} (", table_name).to_string();
    let columns = extract_array(extract_key(map, ":with-columns")?)?;
    let mut string = columns
        .iter()
        .enumerate()
        .try_fold(string, |mut string, (i, column)| {
            let column = extract_array(column.clone())?;
            let name = extract_keyword(column[0].clone())?;
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
                    let type_name = extract_keyword(a[0].clone())?;
                    let type_name = &type_name[1..].to_uppercase();
                    let argument = extract_integer(a[1].clone())?;
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
                        let attribute = extract_keyword(a[0].clone())?;
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
                let name = extract_symbol(name)?;
                let mut new_env = env.clone();
                new_env.insert(name, value);
                Ok((new_env, Expression::Nil))
              }
            ),
            "fn".to_string() => NativeFunction(
              |env, args| {
                let (parameters, body) = (args[0].clone(), args[1].clone());
                let parameters = extract_array(parameters)?;
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
                let mut m = extract_map(map)?;
                m.insert(key, value);
                Ok((env, Expression::Map(m)))
              }
            ),
            "dissoc".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let (map, key) = (args[0].clone(), args[1].clone());
                let mut m = extract_map(map)?;
                m.remove(&key);
                Ok((env, Expression::Map(m)))
              }
            ),
            "merge".to_string() => NativeFunction(
              |env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let (map1, map2) = (args[0].clone(), args[1].clone());
                let mut map1 = extract_map(map1)?;
                let map2 = extract_map(map2)?;
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
                let s = extract_string(arg)?;
                let tokens = crate::Tokens::from_str(&s);
                let expression = crate::parse(tokens);
                Ok((env, expression))
              }
            ),
            "html".to_string() => NativeFunction(|env, args| {
                let (env, args) = evaluate_expressions(env, args)?;
                let mut string = String::new();
                html(args[0].clone(), &mut string)?;
                Ok((env, Expression::String(string)))
            }),
            "server".to_string() => NativeFunction(|env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                let m = extract_map(arg)?;
                let port_expr = m.get(&Expression::Keyword(":port".to_string()));
                let port = match port_expr {
                    Some(Expression::Integer(i)) => i.to_u16().ok_or_else(|| error("Port number out of range"))?,
                    None => 3000,
                    _ => return Err(error("Expected integer for :port")),
                };
                if let Some(tx) = env.servers.lock().get(&port) {
                    tx.send(()).unwrap();
                }
                let mut app = Router::new();
                if let Some(routes) = m.get(&Expression::Keyword(":routes".to_string())) {
                    let m = extract_map(routes.clone())?;
                    for (k, v) in m.iter() {
                        let path = extract_string(k.clone())?;
                        match v.clone() {
                            Expression::String(text) => app = app.route(&path, get(|| async { text })),
                            Expression::Array(_) => {
                                let mut string = String::new();
                                html(v.clone(), &mut string)?;
                                app = app.route(&path, get(|| async { Html(string) }))
                            },
                            _ => return Err(error("Expected string for route")),
                        }
                    }
                }
                let (tx, mut rx) = broadcast::channel(1);
                tokio::spawn(async move {
                    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
                    axum::Server::bind(&socket)
                        .serve(app.into_make_service())
                        .with_graceful_shutdown(async {
                            rx.recv().await.ok();
                        })
                        .await
                        .unwrap();
                });
                {
                    let mut servers = env.servers.lock();
                    servers.insert(port, tx.clone());
                }
                Ok((env, Expression::Nil))
            }),
            "shutdown".to_string() => NativeFunction(|env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                let m = extract_map(arg)?;
                let port_expr = m.get(&Expression::Keyword(":port".to_string()));
                let port = match port_expr {
                    Some(Expression::Integer(i)) => i.to_u16().ok_or_else(|| error("Port number out of range"))?,
                    _ => return Err(error("Expected integer for :port")),
                };
                {
                    let mut servers = env.servers.lock();
                    if let Some(tx) = servers.get(&port) {
                        tx.send(()).unwrap();
                        servers.remove(&port);
                    }
                }
                Ok((env, Expression::Nil))
            }),
            "sqlite".to_string() => NativeFunction(|env, args| {
                let (env, arg) = crate::evaluate(env, args[0].clone())?;
                let path = extract_string(arg)?;
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
        },
        servers: alloc::sync::Arc::new(spin::Mutex::new(HashMap::new())),
    }
}
