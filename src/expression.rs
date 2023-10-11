use crate::numerics::Float;
use im::{HashMap, Vector};
use rug::{Integer, Rational};

type Expressions = Vector<Expression>;

pub type Environment = HashMap<String, Expression>;

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
    IntrinsicFunction(fn(Environment, Expressions) -> Expression),
}
