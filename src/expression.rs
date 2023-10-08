use crate::numerics::Float;
use im::{HashMap, Vector};
use rug::{Integer, Rational};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Array(Vector<Expression>),
    Map(HashMap<Expression, Expression>),
    Call {
        function: Box<Expression>,
        arguments: Vector<Expression>,
    },
}
