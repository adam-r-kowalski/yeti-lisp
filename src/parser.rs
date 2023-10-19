extern crate alloc;

use crate::Expression;
use crate::{tokenizer::Token, Tokens};
use alloc::boxed::Box;
use alloc::string::String;
use core::iter::Peekable;
use im::{HashMap, Vector};

fn symbol(s: String) -> Expression {
    match s.as_ref() {
        "true" => Expression::Bool(true),
        "false" => Expression::Bool(false),
        "nil" => Expression::Nil,
        _ => Expression::Symbol(s),
    }
}

struct Parser<I: Iterator<Item = char>> {
    tokens: Peekable<Tokens<I>>,
}

impl<I: Iterator<Item = char>> Parser<I> {
    fn expression(&mut self) -> Expression {
        match self.tokens.next() {
            Some(Token::Symbol(s)) => symbol(s),
            Some(Token::Keyword(s)) => Expression::Keyword(s),
            Some(Token::String(s)) => Expression::String(s),
            Some(Token::Integer(i)) => Expression::Integer(i),
            Some(Token::Float(f)) => Expression::Float(f),
            Some(Token::Ratio(r)) => Expression::Ratio(r),
            Some(Token::LeftParen) => self.call(),
            Some(Token::LeftBracket) => self.array(),
            Some(Token::LeftBrace) => self.map(),
            Some(Token::Quote) => self.quote(),
            Some(t) => panic!("Unexpected token {:?}", t),
            None => panic!("Expected token got None"),
        }
    }

    fn call(&mut self) -> Expression {
        let function = Box::new(self.expression());
        let mut arguments = Vector::new();
        while let Some(&ref token) = self.tokens.peek() {
            match token {
                Token::RightParen => {
                    self.tokens.next();
                    break;
                }
                _ => {
                    arguments.push_back(self.expression());
                }
            }
        }
        Expression::Call {
            function,
            arguments,
        }
    }

    fn array(&mut self) -> Expression {
        let mut array = Vector::new();
        while let Some(&ref token) = self.tokens.peek() {
            match token {
                Token::RightBracket => {
                    self.tokens.next();
                    break;
                }
                _ => {
                    array.push_back(self.expression());
                }
            }
        }
        Expression::Array(array)
    }

    fn map(&mut self) -> Expression {
        let mut map = HashMap::new();
        while let Some(&ref token) = self.tokens.peek() {
            match token {
                Token::RightBrace => {
                    self.tokens.next();
                    break;
                }
                _ => {
                    let key = self.expression();
                    let value = self.expression();
                    map.insert(key, value);
                }
            }
        }
        Expression::Map(map)
    }

    fn quote(&mut self) -> Expression {
        Expression::Quote(Box::new(self.expression()))
    }
}

pub fn parse<I: Iterator<Item = char>>(tokens: Tokens<I>) -> Expression {
    let mut parser = Parser {
        tokens: tokens.peekable(),
    };
    parser.expression()
}
