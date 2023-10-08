use std::iter::Peekable;
use rug::{Integer, Float};
use im::Vector;
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Array(Vector<Expression>),
    Call{ function: Box<Expression>, arguments: Vector<Expression> },
}

pub fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    match tokens.next() {
        Some(Token::Symbol(s)) => Expression::Symbol(s),
        Some(Token::Keyword(s)) => Expression::Keyword(s),
        Some(Token::String(s)) => Expression::String(s),
        Some(Token::Integer(i)) => Expression::Integer(i),
        Some(Token::Float(f)) => Expression::Float(f),
        Some(Token::LeftParen) => {
            let function = Box::new(parse_expression(tokens));
            let mut arguments = Vector::new();
            while let Some(&ref token) = tokens.peek() {
                match token {
                    Token::RightParen => {
                        tokens.next();
                        break;
                    },
                    _ => {
                        arguments.push_back(parse_expression(tokens));
                    }
                }
            }
            Expression::Call{ function, arguments }
        },
        Some(Token::LeftBracket) => {
            let mut array = Vector::new();
            while let Some(&ref token) = tokens.peek() {
                match token {
                    Token::RightBracket => {
                        tokens.next();
                        break;
                    },
                    _ => {
                        array.push_back(parse_expression(tokens));
                    }
                }
            }
            Expression::Array(array)
        },
        Some(t) => panic!("Unexpected token {:?}", t),
        None => panic!("Expected token got None"),
    }
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let mut tokens = tokens.into_iter().peekable();
    parse_expression(&mut tokens)
}
