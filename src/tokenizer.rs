extern crate alloc;

use crate::numerics::Float;
use crate::peeking_take_while::PeekableExt;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::Peekable;
use core::str::Chars;
use rug::{Integer, Rational};

#[derive(PartialEq, Debug)]
pub enum Token {
    Symbol(String),
    NamespacedSymbol(Vec<String>),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Quote,
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
        let mut string = String::new();
        while let Some(&c) = self.iterator.peek() {
            match c {
                '\\' => {
                    self.iterator.next();
                    match self.iterator.peek() {
                        Some('n') => {
                            self.iterator.next();
                            string.push('\n');
                        }
                        Some('t') => {
                            self.iterator.next();
                            string.push('\t');
                        }
                        Some('r') => {
                            self.iterator.next();
                            string.push('\r');
                        }
                        Some('\\') => {
                            self.iterator.next();
                            string.push('\\');
                        }
                        Some('"') => {
                            self.iterator.next();
                            string.push('"');
                        }
                        Some(_) => {
                            string.push('\\');
                        }
                        None => {
                            string.push('\\');
                        }
                    }
                }
                '"' => break,
                _ => {
                    string.push(c);
                    self.iterator.next();
                }
            }
        }
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
            let numerator = number.parse().unwrap();
            if let Some('/') = self.iterator.peek() {
                self.iterator.next();
                let negative = match self.iterator.peek() {
                    Some('-') => {
                        self.iterator.next();
                        Negative::Yes
                    }
                    _ => Negative::No,
                };
                if let Token::Integer(denominator) = self.number(negative) {
                    let rational = Rational::from((numerator, denominator));
                    if rational.is_integer() {
                        Token::Integer(rational.numer().clone())
                    } else {
                        Token::Ratio(rational)
                    }
                } else {
                    panic!("Expected denominator got {:?}", self.iterator.peek());
                }
            } else {
                Token::Integer(numerator)
            }
        }
    }

    fn symbol(&mut self) -> Token {
        let symbol: String = self
            .iterator
            .peeking_take_while(|&c| !reserved_character(c))
            .collect();
        let parts: Vec<String> = symbol.split('/').map(|s| s.to_string()).collect();
        if parts.len() > 1 {
            Token::NamespacedSymbol(parts)
        } else {
            Token::Symbol(symbol)
        }
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
