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
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    let string: String = chars.peeking_take_while(|&c| c != '"').collect();
    chars.next();
    Token::String(string)
}

fn tokenize_keyword(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    let keyword: String = chars
        .peeking_take_while(|&c| !reserved_character(c))
        .collect();
    Token::Keyword(format!(":{}", keyword))
}

#[derive(PartialEq)]
enum Negative {
    Yes,
    No,
}

fn tokenize_number(chars: &mut Peekable<Chars>, negative: Negative) -> Token {
    let mut is_float = false;
    let mut number: String = chars
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

fn reserved_character(c: char) -> bool {
    match c {
        '(' | ')' | '{' | '}' | '[' | ']' | '"' | ':' => true,
        _ if c.is_whitespace() => true,
        _ => false,
    }
}

fn tokenize_symbol(chars: &mut Peekable<Chars>) -> Token {
    let symbol: String = chars
        .peeking_take_while(|&c| !reserved_character(c))
        .collect();
    Token::Symbol(symbol)
}

fn consume_and_return(chars: &mut Peekable<Chars>, token: Token) -> Token {
    chars.next();
    token
}

pub struct Tokens<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokens {
            chars: input.chars().peekable(),
        }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&c) = self.chars.peek() {
            match c {
                '(' => Some(consume_and_return(&mut self.chars, Token::LeftParen)),
                ')' => Some(consume_and_return(&mut self.chars, Token::RightParen)),
                '{' => Some(consume_and_return(&mut self.chars, Token::LeftBrace)),
                '}' => Some(consume_and_return(&mut self.chars, Token::RightBrace)),
                '[' => Some(consume_and_return(&mut self.chars, Token::LeftBracket)),
                ']' => Some(consume_and_return(&mut self.chars, Token::RightBracket)),
                '\'' => Some(consume_and_return(&mut self.chars, Token::Quote)),
                '"' => Some(tokenize_string(&mut self.chars)),
                ':' => Some(tokenize_keyword(&mut self.chars)),
                '/' => Some(consume_and_return(
                    &mut self.chars,
                    Token::Symbol("/".to_string()),
                )),
                '-' => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&c) if c.is_digit(10) => {
                            Some(tokenize_number(&mut self.chars, Negative::Yes))
                        }
                        _ => Some(Token::Symbol("-".to_string())),
                    }
                }
                _ if c.is_whitespace() => {
                    self.chars.next();
                    self.next()
                }
                _ if c.is_digit(10) => Some(tokenize_number(&mut self.chars, Negative::No)),
                _ => Some(tokenize_symbol(&mut self.chars)),
            }
        } else {
            None
        }
    }
}
