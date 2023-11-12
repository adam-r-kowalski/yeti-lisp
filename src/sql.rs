extern crate alloc;

use crate::effect::{error, Effect};
use crate::expression::{Environment, Module};
use crate::extract;
use crate::numerics::Float;
use crate::Expression::{self, NativeFunction};
use crate::NativeType;
use crate::{evaluate_expressions, evaluate_source};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use im::{ordmap, vector, OrdMap, Vector};
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use rusqlite::{Connection, ToSql};

type Result<T> = core::result::Result<T, Effect>;

fn create_table(map: OrdMap<Expression, Expression>, table_name: Expression) -> Result<Expression> {
    let table_name = &extract::keyword(table_name)?[1..];
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

fn insert_into(map: OrdMap<Expression, Expression>, table_name: Expression) -> Result<Expression> {
    let table_name = &extract::keyword(table_name)?[1..];
    let string = format!("INSERT INTO {} (", table_name).to_string();
    let columns = extract::array(extract::key(map.clone(), ":columns")?)?;
    let mut string = columns
        .iter()
        .enumerate()
        .try_fold(string, |mut string, (i, column)| {
            if i > 0 {
                string.push_str(", ");
            }
            let column = extract::keyword(column.clone())?;
            let column = &column[1..];
            string.push_str(column);
            Ok(string)
        })?;
    string.push_str(") VALUES");
    let placeholder = columns
        .iter()
        .enumerate()
        .fold(String::new(), |mut string, (i, _)| {
            if i > 0 {
                string.push_str(", ");
            }
            string.push_str("?");
            string
        });
    let values = extract::array(extract::key(map, ":values")?)?;
    let string = (0..values.len()).fold(string, |mut string, i| {
        if i > 0 {
            string.push_str(",");
        }
        string.push_str(" (");
        string.push_str(&placeholder);
        string.push(')');
        string
    });
    let result = vector![Expression::String(string)];
    let result = values.iter().try_fold(result, |result, value| {
        let row = extract::array(value.clone())?;
        let result = row.iter().fold(result, |mut result, column| {
            result.push_back(column.clone());
            result
        });
        Ok(result)
    })?;
    Ok(Expression::Array(result))
}

fn select(map: OrdMap<Expression, Expression>, columns: Expression) -> Result<Expression> {
    let columns = match columns {
        Expression::Array(a) => a,
        Expression::Keyword(k) => vector![Expression::Keyword(k)],
        _ => return Err(error("Expected array or keyword")),
    };
    let string = "SELECT".to_string();
    let mut string = columns
        .iter()
        .enumerate()
        .try_fold(string, |mut string, (i, column)| {
            if i > 0 {
                string.push_str(",");
            }
            string.push(' ');
            let column = extract::keyword(column.clone())?;
            string.push_str(&column[1..]);
            Ok(string)
        })?;
    let from = extract::keyword(extract::key(map.clone(), ":from")?)?;
    let from = &from[1..];
    string.push_str(" FROM ");
    string.push_str(from);
    if let Some(where_clause) = map.get(&Expression::Keyword(":where".to_string())) {
        let where_clause = extract::array(where_clause.clone())?;
        let op = extract::keyword(where_clause[0].clone())?;
        let op = match op.as_ref() {
            ":=" => "=",
            ":!=" => "!=",
            ":<" => "<",
            ":<=" => "<=",
            ":>" => ">",
            ":>=" => ">=",
            _ => return Err(error(&format!("Unsupported operator {}", op))),
        };
        let lhs = extract::keyword(where_clause[1].clone())?;
        let lhs = &lhs[1..];
        let rhs = where_clause[2].clone();
        string.push_str(" WHERE ");
        string.push_str(lhs);
        string.push_str(" ");
        string.push_str(op);
        string.push_str(" ?");
        let result = vector![Expression::String(string), rhs];
        Ok(Expression::Array(result))
    } else {
        let result = vector![Expression::String(string)];
        Ok(Expression::Array(result))
    }
}

fn sql_string(expr: Expression) -> Result<Expression> {
    let map = extract::map(expr)?;
    if let Some(table_name) = map.get(&Expression::Keyword(":create-table".to_string())) {
        create_table(map.clone(), table_name.clone())
    } else if let Some(table_name) = map.get(&Expression::Keyword(":insert-into".to_string())) {
        insert_into(map.clone(), table_name.clone())
    } else if let Some(columns) = map.get(&Expression::Keyword(":select".to_string())) {
        select(map.clone(), columns.clone())
    } else {
        Err(error("Unsupported SQL operation"))
    }
}

pub fn connect(env: Environment, _args: Vector<Expression>) -> Result<(Environment, Expression)> {
    match Connection::open_in_memory() {
        Ok(db) => Ok((
            env,
            Expression::NativeType(NativeType::new(db, "sqlite".to_string())),
        )),
        Err(_) => Err(error("Failed to open SQLite database")),
    }
}

pub fn string(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let expr = sql_string(args[0].clone())?;
    Ok((env, expr))
}

impl ToSql for Expression {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        match self {
            Expression::Integer(i) => Ok(ToSqlOutput::Owned(Value::Integer(i.to_i64().unwrap()))),
            Expression::Float(f) => Ok(ToSqlOutput::Owned(Value::Real(f.to_f64()))),
            Expression::Ratio(r) => Ok(ToSqlOutput::Owned(Value::Real(r.to_f64()))),
            Expression::String(s) => Ok(ToSqlOutput::Owned(Value::Text(s.clone()))),
            Expression::Nil => Ok(ToSqlOutput::Owned(Value::Null)),
            Expression::Bool(b) => Ok(ToSqlOutput::Owned(Value::Integer(if *b { 1 } else { 0 }))),
            _ => {
                let effect = error(&format!("Unsupported data type: {:?}", self));
                Err(rusqlite::Error::ToSqlConversionFailure(Box::new(effect)))
            }
        }
    }
}

impl FromSql for Expression {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(Expression::Nil),
            ValueRef::Integer(int) => Ok(Expression::Integer(rug::Integer::from(int))),
            ValueRef::Real(float) => Ok(Expression::Float(Float::from_f64(float))),
            ValueRef::Text(text) => Ok(Expression::String(
                String::from_utf8_lossy(text).into_owned(),
            )),
            ValueRef::Blob(blob) => Ok(Expression::String(
                String::from_utf8_lossy(blob).into_owned(),
            )),
        }
    }
}

pub fn query(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let db = extract::native_type(args[0].clone())?;
    let array = extract::array(sql_string(args[1].clone())?)?;
    let string = extract::string(array[0].clone())?;
    let parameters = array
        .iter()
        .skip(1)
        .map(|p| p as &dyn ToSql)
        .collect::<Vec<_>>();
    let connection = db.value.lock();
    let connection = connection
        .downcast_ref::<Connection>()
        .ok_or_else(|| error("Expected SQLite database connection"))?;
    let result = connection.prepare(&string);
    match result {
        Ok(mut stmt) => {
            let column_names: Vec<String> =
                stmt.column_names().iter().map(|c| c.to_string()).collect();
            let rows: Vector<Expression> = stmt
                .query_map(&parameters[..], |row| {
                    let result = column_names.iter().enumerate().try_fold(
                        OrdMap::new(),
                        |mut map, (i, name)| {
                            let value: Expression = row.get(i)?;
                            map.insert(Expression::Keyword(format!(":{}", name)), value);
                            Ok(map)
                        },
                    );
                    match result {
                        Ok(map) => Ok(Expression::Map(map)),
                        Err(e) => Err(e),
                    }
                })
                .or_else(|e| Err(error(&format!("Failed to execute query: {}", e))))?
                .map(|row| row.unwrap())
                .collect();
            Ok((env, Expression::Array(rows)))
        }
        Err(e) => Err(error(&format!("Failed to execute query: {}", e))),
    }
}

pub fn execute(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args)?;
    let db = extract::native_type(args[0].clone())?;
    let array = extract::array(sql_string(args[1].clone())?)?;
    let string = extract::string(array[0].clone())?;
    let parameters = array
        .iter()
        .skip(1)
        .map(|p| p as &dyn ToSql)
        .collect::<Vec<_>>();
    let connection = db.value.lock();
    let connection = connection
        .downcast_ref::<Connection>()
        .ok_or_else(|| error("Expected SQLite database connection"))?;
    match connection.execute(&string, &parameters[..]) {
        Ok(_) => Ok((env, Expression::Nil)),
        Err(e) => Err(error(&format!("Failed to execute query: {}", e))),
    }
}

pub fn tables(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let db = args[0].clone();
    let (env, q) = evaluate_source(
        env,
        r#"
        {:select [:name]
         :from :sqlite_master
         :where [:= :type "table"]}
        "#,
    )?;
    query(env, vector![db, q])
}

pub fn module() -> Module {
    Module {
        name: "sql".to_string(),
        environment: ordmap! {
            "connect".to_string() => NativeFunction(connect),
            "string".to_string() => NativeFunction(string),
            "query".to_string() => NativeFunction(query),
            "execute!".to_string() => NativeFunction(execute),
            "tables".to_string() => NativeFunction(tables)
        },
    }
}
