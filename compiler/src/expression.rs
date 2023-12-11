extern crate alloc;

use crate::atom::Atom;
use crate::channel::Channel;
use crate::effect::Effect;
use crate::numerics::Float;
use crate::NativeType;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
use core::future::Future;
use core::hash::Hash;
use core::pin::Pin;
use im::{OrdMap, Vector};
use rug::{Integer, Rational};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer};

type Expressions = Vector<Expression>;

pub type Environment = OrdMap<String, Expression>;

pub type Result = core::result::Result<(Environment, Expression), Effect>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pattern {
    pub parameters: Expressions,
    pub body: Vector<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Function {
    pub env: Environment,
    pub patterns: Vector<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Call {
    pub function: Box<Expression>,
    pub arguments: Expressions,
}

type NativeFunction = fn(Environment, Expressions) -> Pin<Box<dyn Future<Output = Result> + Send>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expression {
    Symbol(String),
    NamespacedSymbol(Vec<String>),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Bool(bool),
    Nil,
    Array(Expressions),
    Map(OrdMap<Expression, Expression>),
    Call(Call),
    Function(Function),
    Quote(Box<Expression>),
    Deref(Box<Expression>),
    Atom(Atom),
    Channel(Channel),
    NativeFunction(NativeFunction),
    NativeType(NativeType),
    Module(Environment),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {} {}", k, v)?;
                    } else {
                        write!(f, "{} {}", k, v)?;
                    }
                }
                write!(f, "}}")
            }
            Expression::Array(arr) => {
                write!(f, "[")?;
                for (i, e) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", e)?;
                    } else {
                        write!(f, "{}", e)?;
                    }
                }
                write!(f, "]")
            }
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::NamespacedSymbol(s) => write!(f, "{}", s.join("/")),
            Expression::Keyword(k) => write!(f, "{}", k),
            Expression::String(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '\n' => write!(f, "\\n")?,
                        '\t' => write!(f, "\\t")?,
                        '\r' => write!(f, "\\r")?,
                        '\\' => write!(f, "\\\\")?,
                        '"' => write!(f, "\\\"")?,
                        _ => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(fl) => write!(f, "{}", fl),
            Expression::Ratio(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            Expression::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Expression::Nil => write!(f, "nil"),
            Expression::Call(Call {
                function,
                arguments,
            }) => {
                let arg_strs: Vec<String> = arguments.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({} {})", function, arg_strs.join(" "))
            }
            Expression::Function(function) => {
                if function.patterns.len() == 1 {
                    let Pattern { parameters, body } = &function.patterns[0];
                    let param_strs: Vec<String> =
                        parameters.iter().map(|e| format!("{}", e)).collect();
                    write!(f, "(fn [{}] ", param_strs.join(" "))?;
                    body.iter().try_for_each(|e| write!(f, "{} ", e))?;
                    write!(f, ")")
                } else {
                    write!(f, "(fn")?;
                    for Pattern { parameters, body } in &function.patterns {
                        let param_strs: Vec<String> =
                            parameters.iter().map(|e| format!("{}", e)).collect();
                        write!(f, "\n  ([{}] ", param_strs.join(" "))?;
                        body.iter().try_for_each(|e| write!(f, "{} ", e))?;
                        write!(f, ")")?;
                    }
                    write!(f, ")")
                }
            }
            Expression::NativeFunction(_) => write!(f, "#native_function"),
            Expression::NativeType(t) => write!(f, "{}", t),
            Expression::Atom(a) => write!(f, "{}", a),
            Expression::Channel(c) => write!(f, "{}", c),
            Expression::Quote(e) => write!(f, "'{}", e),
            Expression::Deref(e) => write!(f, "@{}", e),
            Expression::Module(e) => write!(
                f,
                "#module({})",
                e.get("*name*").unwrap_or(&Expression::Nil)
            ),
        }
    }
}
impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Expression::Keyword(s) => s[1..].serialize(serializer),
            Expression::String(s) => s.serialize(serializer),
            Expression::Integer(i) => i.to_i64().serialize(serializer),
            Expression::Float(f) => f.to_f64().serialize(serializer),
            Expression::Ratio(r) => r.to_f64().serialize(serializer),
            Expression::Bool(b) => b.serialize(serializer),
            Expression::Nil => serializer.serialize_unit(),
            Expression::Array(expressions) => {
                let mut seq = serializer.serialize_seq(Some(expressions.len()))?;
                for e in expressions {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Expression::Map(map) => {
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(k, v)?;
                }
                map_ser.end()
            }
            _ => Err(serde::ser::Error::custom("unsupported variant")),
        }
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExpressionVisitor)
    }
}

struct ExpressionVisitor;

impl<'de> Visitor<'de> for ExpressionVisitor {
    type Value = Expression;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid JSON value")
    }

    fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::String(v.to_string()))
    }

    fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::Integer(Integer::from(v)))
    }

    fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::Integer(Integer::from(v)))
    }

    fn visit_f64<E>(self, v: f64) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::Float(Float::from_f64(v)))
    }

    fn visit_bool<E>(self, v: bool) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::Bool(v))
    }

    fn visit_seq<V>(self, mut seq: V) -> core::result::Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut expressions = Vector::new();
        while let Some(elem) = seq.next_element()? {
            expressions.push_back(elem);
        }
        Ok(Expression::Array(expressions))
    }

    fn visit_map<V>(self, mut map: V) -> core::result::Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut ord_map = OrdMap::new();
        while let Some((key, value)) = map.next_entry()? {
            let key = match key {
                Expression::String(s) => Expression::Keyword(format!(":{}", s)),
                _ => key,
            };
            ord_map.insert(key, value);
        }
        Ok(Expression::Map(ord_map))
    }

    fn visit_unit<E>(self) -> core::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Expression::Nil)
    }
}
