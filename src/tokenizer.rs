use crate::numerics::Float;
use crate::peeking_take_while::PeekableExt;
use rug::Integer;
use std::iter::Peekable;
use std::str::Chars;

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

fn next_token(mut chars: &mut Peekable<Chars>) -> Option<Token> {
    if let Some(&c) = chars.peek() {
        match c {
            '(' => Some(consume_and_return(chars, Token::LeftParen)),
            ')' => Some(consume_and_return(chars, Token::RightParen)),
            '{' => Some(consume_and_return(chars, Token::LeftBrace)),
            '}' => Some(consume_and_return(chars, Token::RightBrace)),
            '[' => Some(consume_and_return(chars, Token::LeftBracket)),
            ']' => Some(consume_and_return(chars, Token::RightBracket)),
            '"' => Some(tokenize_string(&mut chars)),
            ':' => Some(tokenize_keyword(&mut chars)),
            '-' => {
                chars.next();
                match chars.peek() {
                    Some(&c) if c.is_digit(10) => Some(tokenize_number(&mut chars, Negative::Yes)),
                    _ => Some(Token::Symbol("-".to_string())),
                }
            }
            _ if c.is_whitespace() => {
                chars.next();
                next_token(chars)
            }
            _ if c.is_digit(10) => Some(tokenize_number(&mut chars, Negative::No)),
            _ => Some(tokenize_symbol(&mut chars)),
        }
    } else {
        None
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(token) = next_token(&mut chars) {
        tokens.push(token);
    }
    tokens
}
