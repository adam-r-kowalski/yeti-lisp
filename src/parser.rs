use std::iter::Peekable;
use rug::{Integer, Float};
use im::Vector;
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Integer(Integer),
    Float(Float),
    Array(Vector<Expression>),
}

pub fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    match tokens.next() {
        Some(Token::Integer(i)) => Expression::Integer(i),
        Some(Token::Float(f)) => Expression::Float(f),
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
