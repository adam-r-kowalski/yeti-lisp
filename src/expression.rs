extern crate alloc;

use crate::effect::Effect;
use crate::numerics::Float;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
use core::hash::Hash;
use im::{HashMap, Vector};
use rug::{Integer, Rational};
use rusqlite::Connection;
use spin::Mutex;
use tokio::sync::broadcast::Sender;

type Expressions = Vector<Expression>;

#[derive(Debug, Clone)]
pub struct Environment {
    pub bindings: HashMap<String, Expression>,
    pub servers: Arc<Mutex<HashMap<u16, Sender<()>>>>,
}

impl Environment {
    pub fn get(&self, key: &str) -> Option<Expression> {
        self.bindings.get(key).cloned()
    }

    pub fn insert(&mut self, key: String, value: Expression) {
        self.bindings.insert(key, value);
    }

    pub fn new() -> Environment {
        Environment {
            bindings: HashMap::new(),
            servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
pub type Result = core::result::Result<(Environment, Expression), Effect>;

pub struct Sqlite {
    pub connection: Arc<Connection>,
}

impl Sqlite {
    pub fn new(connection: Connection) -> Sqlite {
        Sqlite {
            connection: Arc::new(connection),
        }
    }
}

impl PartialEq for Sqlite {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.connection, &other.connection)
    }
}

impl core::hash::Hash for Sqlite {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.connection).hash(state)
    }
}

impl core::fmt::Debug for Sqlite {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#sqlite({:?})", Arc::as_ptr(&self.connection))
    }
}

impl Eq for Sqlite {}

impl Clone for Sqlite {
    fn clone(&self) -> Self {
        Sqlite {
            connection: self.connection.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Bool(bool),
    Nil,
    Array(Expressions),
    Map(HashMap<Expression, Expression>),
    Call {
        function: Box<Expression>,
        arguments: Expressions,
    },
    Function {
        parameters: Expressions,
        body: Box<Expression>,
    },
    NativeFunction(fn(Environment, Expressions) -> Result),
    Sqlite(Sqlite),
    Quote(Box<Expression>),
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
            Expression::Keyword(k) => write!(f, "{}", k),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(fl) => write!(f, "{}", fl),
            Expression::Ratio(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            Expression::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Expression::Nil => write!(f, "nil"),
            Expression::Call {
                function,
                arguments,
            } => {
                let arg_strs: Vec<String> = arguments.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({} {})", function, arg_strs.join(" "))
            }
            Expression::Function { parameters, body } => {
                let param_strs: Vec<String> = parameters.iter().map(|e| format!("{}", e)).collect();
                write!(f, "(fn [{}] {})", param_strs.join(" "), body)
            }
            Expression::NativeFunction(_) => write!(f, "#native_function"),
            Expression::Sqlite(s) => write!(f, "{:?}", s),
            Expression::Quote(e) => write!(f, "'{}", e),
        }
    }
}
