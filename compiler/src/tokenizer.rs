extern crate alloc;

use crate::numerics::Float;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
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
    Deref,
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

fn push(mut tokens: Vec<Token>, token: Token) -> Vec<Token> {
    tokens.push(token);
    tokens
}

fn rest(input: &str) -> &str {
    match input.get(1..) {
        Some(rest) => rest,
        None => "",
    }
}

fn string(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    let mut chars = input.chars();
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('n') => string.push('\n'),
                Some('t') => string.push('\t'),
                Some('r') => string.push('\r'),
                Some('\\') => string.push('\\'),
                Some('"') => string.push('"'),
                Some(other) => string.push_str(&format!("\\{}", other)),
                None => string.push('\\'),
            },
            '"' => break,
            _ => string.push(c),
        }
    }
    (chars.as_str(), push(tokens, Token::String(string)))
}

fn keyword(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    let mut chars = input.chars();
    let mut keyword = String::new();
    loop {
        match chars.clone().next() {
            None => break,
            Some(c) if reserved_character(c) => break,
            Some(c) => {
                keyword.push(c);
                chars.next();
            }
        }
    }
    (chars.as_str(), push(tokens, Token::Keyword(format!(":{}", keyword))))
}

fn comment(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\n' { break; }
    }
    (chars.as_str(), tokens)
}


fn symbol(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    let mut chars = input.chars();
    let mut symbol = String::new();
    loop {
        match chars.clone().next() {
            None => break,
            Some(c) if reserved_character(c) => break,
            Some(c) => {
                symbol.push(c);
                chars.next();
            }
        }
    }
    let parts: Vec<String> = symbol.split('/').map(|s| s.to_string()).collect();
    let token = if parts.len() > 1 {
        Token::NamespacedSymbol(parts)
    } else {
        Token::Symbol(symbol)
    };
    (chars.as_str(), push(tokens, token))
}

fn number(input: &str, mut tokens: Vec<Token>, negative: Negative) -> (&str, Vec<Token>) {
    let mut chars = input.chars();
    let mut number_string = String::new();
    let mut is_float = false;
    while let Some(c) = chars.clone().next() {
        match c {
            '.' if !is_float => {
                is_float = true;
                number_string.push(c);
                chars.next();
            }
            _ if c.is_digit(10) || c == '_' => {
                if c != '_' { number_string.push(c); }
                chars.next();
            }
            _ => break,
        }
    }
    if negative == Negative::Yes {
        number_string.insert(0, '-');
    }
    let token = if is_float {
        Token::Float(Float::from_str(&number_string))
    } else {
        let numerator = number_string.parse::<Integer>().unwrap();
        match chars.clone().next() {
            Some('/') => {
                chars.next();
                let negative = match chars.clone().next() {
                    Some('-') => {
                        chars.next();
                        Negative::Yes
                    }
                    _ => Negative::No,
                };
                let (input, new_tokens) = number(chars.as_str(), tokens, negative);
                chars = input.chars();
                tokens = new_tokens;
                if let Some(Token::Integer(denominator)) = tokens.pop() {
                    let rational = Rational::from((numerator, denominator.clone()));
                    if rational.is_integer() {
                        Token::Integer(rational.numer().clone())
                    } else {
                        Token::Ratio(rational)
                    }
                } else {
                    panic!("Expected denominator after '/'");
                }
            }
            _ => Token::Integer(numerator),
        }
    };
    (chars.as_str(), push(tokens, token))
}

fn negative_number_or_symbol(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    match input.chars().peekable().peek() {
        Some(&c) if c.is_digit(10) => number(input, tokens, Negative::Yes),
        _ => {
            let (input, mut tokens) = symbol(input, tokens);
            if let Some(Token::Symbol(symbol)) = tokens.pop() {
                (input, push(tokens, Token::Symbol(format!("-{}", symbol))))
            } else {
                panic!("Expected symbol got {:?}", input.chars().peekable().peek());
            }
        }
    }
}

fn next(input: &str, tokens: Vec<Token>) -> (&str, Vec<Token>) {
    match input.chars().next() {
        Some('@') => (rest(input), push(tokens, Token::Deref)),
        Some('(') => (rest(input), push(tokens, Token::LeftParen)),
        Some(')') => (rest(input), push(tokens, Token::RightParen)),
        Some('{') => (rest(input), push(tokens, Token::LeftBrace)),
        Some('}') => (rest(input), push(tokens, Token::RightBrace)),
        Some('[') => (rest(input), push(tokens, Token::LeftBracket)),
        Some(']') => (rest(input), push(tokens, Token::RightBracket)),
        Some('\'') => (rest(input), push(tokens, Token::Quote)),
        Some('"') => string(rest(input), tokens),
        Some(':') => keyword(rest(input), tokens),
        Some(';') => comment(rest(input), tokens),
        Some('/') => (rest(input), push(tokens, Token::Symbol("/".to_string()))),
        Some('-') => negative_number_or_symbol(rest(input), tokens),
        Some(c) if is_whitespace(c) => (rest(input), tokens),
        Some(c) if c.is_digit(10) => number(input, tokens, Negative::No),
        Some(_) => symbol(input, tokens),
        None => (input, tokens),
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input = input;
    while !input.is_empty() {
        (input, tokens) = next(input, tokens);
    }
    tokens
}

