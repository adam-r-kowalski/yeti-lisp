use std::fmt;
use std::iter::Peekable;
use rug::{Integer, Float, Rational};
use im::Vector;
use crate::tokenizer::Token;
use crate::numerics::bits_to_decimal_digits;

#[derive(PartialEq, Clone)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Array(Vector<Expression>),
    Call{ function: Box<Expression>, arguments: Vector<Expression> },
}

impl fmt::Debug for Expression {
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
            },
            Expression::Ratio(ratio) => write!(f, "{}/{}", ratio.numer(), ratio.denom()),
            Expression::Array(array) => {
                write!(f, "[")?;
                for (i, element) in array.iter().enumerate() {
                    if i > 0 { write!(f, " ")?; }
                    write!(f, "{:?}", element)?;
                }
                write!(f, "]")
            },
            Expression::Call{ function, arguments } => {
                write!(f, "({:?}", function)?;
                for argument in arguments {
                    write!(f, " {:?}", argument)?;
                }
                write!(f, ")")
            },
        }
    }
}

fn parse_integer<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>, i: Integer) -> Expression {
    match tokens.peek() {
        Some(&Token::Symbol(ref s)) if s == "/" => {
            tokens.next();
            let denominator = match tokens.next() {
                Some(Token::Integer(i)) => i,
                Some(t) => panic!("Expected integer got {:?}", t),
                None => panic!("Expected integer got None"),
            };
            Expression::Ratio(Rational::from((i, denominator)))
        },
        _ => Expression::Integer(i)
    }
}

fn parse_call<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let function = Box::new(parse_expression(tokens));
    let mut arguments = Vector::new();
    while let Some(&ref token) = tokens.peek() {
        match token {
            Token::RightParen => {
                tokens.next();
                break;
            },
            _ => { arguments.push_back(parse_expression(tokens)); }
        }
    }
    Expression::Call{ function, arguments }
}

fn parse_array<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut array = Vector::new();
    while let Some(&ref token) = tokens.peek() {
        match token {
            Token::RightBracket => {
                tokens.next();
                break;
            },
            _ => { array.push_back(parse_expression(tokens)); }
        }
    }
    Expression::Array(array)
}

pub fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    match tokens.next() {
        Some(Token::Symbol(s)) => Expression::Symbol(s),
        Some(Token::Keyword(s)) => Expression::Keyword(s),
        Some(Token::String(s)) => Expression::String(s),
        Some(Token::Integer(i)) => parse_integer(tokens, i),
        Some(Token::Float(f)) => Expression::Float(f),
        Some(Token::LeftParen) => parse_call(tokens),
        Some(Token::LeftBracket) => parse_array(tokens),
        Some(t) => panic!("Unexpected token {:?}", t),
        None => panic!("Expected token got None"),
    }
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let mut tokens = tokens.into_iter().peekable();
    parse_expression(&mut tokens)
}
