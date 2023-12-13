extern crate alloc;

use crate::expression::Call;
use crate::Expression;
use crate::tokenizer::Token;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use im::{OrdMap, Vector};

fn symbol(s: String) -> Expression {
    match s.as_ref() {
        "true" => Expression::Bool(true),
        "false" => Expression::Bool(false),
        "nil" => Expression::Nil,
        _ => Expression::Symbol(s),
    }
}

fn call(tokens: &[Token]) -> (&[Token], Expression) {
    let (mut tokens, function) = parse(tokens);
    let function = Box::new(function);
    let mut arguments = Vector::new();
    while let Some(&ref token) = tokens.get(0) {
        match token {
            Token::RightParen => {
                tokens = rest(tokens);
                break;
            }
            _ => {
                let (new_tokens, argument) = parse(tokens);
                tokens = new_tokens;
                arguments.push_back(argument);
            }
        }
    }
    (tokens, Expression::Call(Call {
        function,
        arguments,
    }))
}

fn array(mut tokens: &[Token]) -> (&[Token], Expression) {
    let mut array = Vector::new();
    while let Some(&ref token) = tokens.get(0) {
        match token {
            Token::RightBracket => {
                tokens = rest(tokens);
                break;
            }
            _ => {
                let (new_tokens, expression) = parse(tokens);
                tokens = new_tokens;
                array.push_back(expression);
            }
        }
    }
    (tokens, Expression::Array(array))
}

fn map(mut tokens: &[Token]) -> (&[Token], Expression) {
    let mut map = OrdMap::new();
    while let Some(&ref token) = tokens.get(0) {
        match token {
            Token::RightBrace => {
                tokens = rest(tokens);
                break;
            }
            _ => {
                let (new_tokens, key) = parse(tokens);
                let (new_tokens, value) = parse(new_tokens);
                tokens = new_tokens;
                map.insert(key, value);
            }
        }
    }
    (tokens, Expression::Map(map))
}

fn quote(tokens: &[Token]) -> (&[Token], Expression) {
    let (tokens, expression) = parse(tokens);
    (tokens, Expression::Quote(Box::new(expression)))
}

fn deref(tokens: &[Token]) -> (&[Token], Expression) {
    let (tokens, expression) = parse(tokens);
    (tokens, Expression::Deref(Box::new(expression)))
}

fn rest(tokens: &[Token]) -> &[Token] {
    match tokens.get(1..) {
        Some(rest) => rest,
        None => &[],
    }
}

pub fn parse(tokens: &[Token]) -> (&[Token], Expression) {
    match tokens.get(0) {
        Some(Token::Symbol(s)) => (rest(tokens), symbol(s.clone())),
        Some(Token::NamespacedSymbol(s)) => (rest(tokens), Expression::NamespacedSymbol(s.clone())),
        Some(Token::Keyword(s)) => (rest(tokens), Expression::Keyword(s.clone())),
        Some(Token::String(s)) => (rest(tokens), Expression::String(s.clone())),
        Some(Token::Integer(i)) => (rest(tokens), Expression::Integer(i.clone())),
        Some(Token::Float(f)) => (rest(tokens), Expression::Float(f.clone())),
        Some(Token::Ratio(r)) => (rest(tokens), Expression::Ratio(r.clone())),
        Some(Token::LeftParen) => call(rest(tokens)),
        Some(Token::LeftBracket) => array(rest(tokens)),
        Some(Token::LeftBrace) => map(rest(tokens)),
        Some(Token::Quote) => quote(rest(tokens)),
        Some(Token::Deref) => deref(rest(tokens)),
        Some(t) => panic!("Unexpected token {:?}", t),
        None => panic!("Expected token got None"),
    }

}

pub fn parse_all(mut tokens: &[Token]) -> Vec<Expression> {
    let mut expressions = Vec::new();
    while !tokens.is_empty() {
        let (new_tokens, expression) = parse(tokens);
        tokens = new_tokens;
        expressions.push(expression);
    }
    expressions
}
