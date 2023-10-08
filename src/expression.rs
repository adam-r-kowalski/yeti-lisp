use crate::numerics::bits_to_decimal_digits;
use im::Vector;
use rug::{Float, Integer, Rational};
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Array(Vector<Expression>),
    Call {
        function: Box<Expression>,
        arguments: Vector<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Keyword(k) => write!(f, "{}", k),
            Expression::String(s) => write!(f, "{}", s),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(float) => {
                let bits = float.prec();
                let digits = bits_to_decimal_digits(bits);
                write!(f, "{:.*}", digits, float)
            }
            Expression::Ratio(ratio) => write!(f, "{}/{}", ratio.numer(), ratio.denom()),
            Expression::Array(array) => {
                write!(f, "[")?;
                for (i, element) in array.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", element)?;
                }
                write!(f, "]")
            }
            Expression::Call {
                function,
                arguments,
            } => {
                write!(f, "({}", function)?;
                for argument in arguments {
                    write!(f, " {}", argument)?;
                }
                write!(f, ")")
            }
        }
    }
}
