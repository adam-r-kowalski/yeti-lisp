use crate::numerics::Float;
use im::{HashMap, Vector};
use rug::{Integer, Rational};
use std::fmt::{self, Display, Formatter};

type Expressions = Vector<Expression>;

pub type Environment = HashMap<String, Expression>;

#[derive(Debug)]
pub struct RaisedEffect {
    pub environment: Environment,
    pub effect: String,
    pub arguments: Vector<Expression>,
}

pub type Result = std::result::Result<(Environment, Expression), RaisedEffect>;

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
    IntrinsicFunction(fn(Environment, Expressions) -> Result),
    Quote(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Keyword(k) => write!(f, "{}", k),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(fl) => write!(f, "{}", fl),
            Expression::Ratio(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            Expression::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Expression::Nil => write!(f, "nil"),
            Expression::Array(arr) => {
                let strs: Vec<String> = arr.iter().map(|e| format!("{}", e)).collect();
                write!(f, "[{}]", strs.join(" "))
            }
            Expression::Map(map) => {
                let strs: Vec<String> = map.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
                write!(f, "{{{}}}", strs.join(", "))
            }
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
            Expression::IntrinsicFunction(_) => write!(f, "#intrinsic"),
            Expression::Quote(e) => write!(f, "'{}", e),
        }
    }
}
