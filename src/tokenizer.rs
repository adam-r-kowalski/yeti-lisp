extern crate alloc;

use crate::numerics::Float;
use crate::peeking_take_while::PeekableExt;
use alloc::format;
use alloc::string::{String, ToString};
use core::iter::Peekable;
use core::str::Chars;
use rug::Integer;

#[derive(PartialEq, Debug)]
pub enum Token {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Quote,
    NewLine,
}

#[derive(PartialEq)]
enum Negative {
    Yes,
    No,
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ','
}

fn reserved_character(c: char) -> bool {
    match c {
        '(' | ')' | '{' | '}' | '[' | ']' | '"' | ':' => true,
        _ if is_whitespace(c) => true,
        _ => false,
    }
}

pub struct Tokens<I: Iterator<Item = char>> {
    iterator: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokens<I> {
    pub fn new(iterator: I) -> Self {
        Tokens {
            iterator: iterator.peekable(),
        }
    }
}

impl<'a> Tokens<Chars<'a>> {
    pub fn from_str(input: &'a str) -> Self {
        Tokens {
            iterator: input.chars().peekable(),
        }
    }
}

impl<I: Iterator<Item = char>> Tokens<I> {
    fn consume_and_return(&mut self, token: Token) -> Token {
        self.iterator.next();
        token
    }

    fn string(&mut self) -> Token {
        self.iterator.next();
        let string: String = self.iterator.peeking_take_while(|&c| c != '"').collect();
        self.iterator.next();
        Token::String(string)
    }

    fn keyword(&mut self) -> Token {
        self.iterator.next();
        let keyword: String = self
            .iterator
            .peeking_take_while(|&c| !reserved_character(c))
            .collect();
        Token::Keyword(format!(":{}", keyword))
    }

    fn number(&mut self, negative: Negative) -> Token {
        let mut is_float = false;
        let mut number: String = self
            .iterator
            .peeking_take_while(|&c| {
                if c == '.' {
                    is_float = true;
                }
                c.is_digit(10) || c == '_' || c == '.'
            })
            .filter(|&c| c != '_')
            .collect();
        if negative == Negative::Yes {
            number = format!("-{}", number);
        }
        if is_float {
            Token::Float(Float::from_str(&number))
        } else {
            Token::Integer(number.parse().unwrap())
        }
    }

    fn symbol(&mut self) -> Token {
        let symbol: String = self
            .iterator
            .peeking_take_while(|&c| !reserved_character(c))
            .collect();
        Token::Symbol(symbol)
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokens<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&c) = self.iterator.peek() {
            match c {
                '(' => Some(self.consume_and_return(Token::LeftParen)),
                ')' => Some(self.consume_and_return(Token::RightParen)),
                '{' => Some(self.consume_and_return(Token::LeftBrace)),
                '}' => Some(self.consume_and_return(Token::RightBrace)),
                '[' => Some(self.consume_and_return(Token::LeftBracket)),
                ']' => Some(self.consume_and_return(Token::RightBracket)),
                '\'' => Some(self.consume_and_return(Token::Quote)),
                '"' => Some(self.string()),
                ':' => Some(self.keyword()),
                '/' => Some(self.consume_and_return(Token::Symbol("/".to_string()))),
                '-' => {
                    self.iterator.next();
                    match self.iterator.peek() {
                        Some(&c) if c.is_digit(10) => Some(self.number(Negative::Yes)),
                        _ => Some(Token::Symbol("-".to_string())),
                    }
                }
                '\n' => Some(self.consume_and_return(Token::NewLine)),
                _ if is_whitespace(c) => {
                    self.iterator.next();
                    self.next()
                }
                _ if c.is_digit(10) => Some(self.number(Negative::No)),
                _ => Some(self.symbol()),
            }
        } else {
            None
        }
    }
}
