use crate::tokenizer::Token;
use crate::Expression;
use im::{HashMap, Vector};
use rug::{Integer, Rational};
use std::iter::Peekable;

fn parse_symbol(s: String) -> Expression {
    match s.as_ref() {
        "true" => Expression::Bool(true),
        "false" => Expression::Bool(false),
        "nil" => Expression::Nil,
        _ => Expression::Symbol(s),
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
        }
        _ => Expression::Integer(i),
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
            }
            _ => {
                arguments.push_back(parse_expression(tokens));
            }
        }
    }
    Expression::Call {
        function,
        arguments,
    }
}

fn parse_array<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut array = Vector::new();
    while let Some(&ref token) = tokens.peek() {
        match token {
            Token::RightBracket => {
                tokens.next();
                break;
            }
            _ => {
                array.push_back(parse_expression(tokens));
            }
        }
    }
    Expression::Array(array)
}

fn parse_map<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    let mut map = HashMap::new();
    while let Some(&ref token) = tokens.peek() {
        match token {
            Token::RightBrace => {
                tokens.next();
                break;
            }
            _ => {
                let key = parse_expression(tokens);
                let value = parse_expression(tokens);
                map.insert(key, value);
            }
        }
    }
    Expression::Map(map)
}

pub fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Expression {
    match tokens.next() {
        Some(Token::Symbol(s)) => parse_symbol(s),
        Some(Token::Keyword(s)) => Expression::Keyword(s),
        Some(Token::String(s)) => Expression::String(s),
        Some(Token::Integer(i)) => parse_integer(tokens, i),
        Some(Token::Float(f)) => Expression::Float(f),
        Some(Token::LeftParen) => parse_call(tokens),
        Some(Token::LeftBracket) => parse_array(tokens),
        Some(Token::LeftBrace) => parse_map(tokens),
        Some(t) => panic!("Unexpected token {:?}", t),
        None => panic!("Expected token got None"),
    }
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let mut tokens = tokens.into_iter().peekable();
    parse_expression(&mut tokens)
}
