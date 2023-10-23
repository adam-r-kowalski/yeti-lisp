extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate_expressions;
use crate::expression::{Environment, Sqlite};
use crate::extract;
use crate::Expression;
use alloc::format;
use alloc::string::ToString;
use im::{vector, Vector};
use rusqlite::Connection;

type Result<T> = core::result::Result<T, Effect>;

fn sql_string(expr: Expression) -> Result<Expression> {
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

pub fn sqlite(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
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
}

pub fn sql(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let expr = sql_string(args[0].clone())?;
    Ok((env, expr))
}
