use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use rug::{Integer, Float};

fn decimal_digits_to_bits(decimal_digits: usize) -> u32 {
    (decimal_digits as f64 * 3.322).ceil() as u32
}

fn bits_to_decimal_digits(bits: u32) -> usize {
    (bits as f64 / 3.322).floor() as usize
}

pub fn string_to_float(number: &str) -> Float {
    let offset = if number.starts_with("-") { 2 } else { 1 };
    let digits = number.len() - offset;
    let bits = decimal_digits_to_bits(digits);
    let parsed = Float::parse(number).unwrap();
    Float::with_val(bits, parsed)
}


#[derive(PartialEq)]
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Symbol(s) => write!(f, "Symbol({})", s),
            Token::Keyword(k) => write!(f, "Keyword({})", k),
            Token::String(s) => write!(f, "String({:?})", s),
            Token::Integer(i) => write!(f, "Integer({})", i),
            Token::Float(float) => {
                let bits = float.prec();
                let digits = bits_to_decimal_digits(bits);
                write!(f, "Float({:.*})", digits, float)
            },
            Token::LeftParen => write!(f, "LeftParen"),
            Token::RightParen => write!(f, "RightParen"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
        }
    }
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    let string: String = chars
        .by_ref()
        .take_while(|&c| c != '"')
        .collect();
    chars.next();
    Token::String(string)
}

fn tokenize_keyword(chars: &mut Peekable<Chars>) -> Token {
    let keyword: String = chars
        .by_ref()
        .take_while(|&c| !c.is_whitespace())
        .collect();
    Token::Keyword(keyword)
}

#[derive(PartialEq)]
enum Negative {
    Yes,
    No,
}

fn tokenize_number(chars: &mut Peekable<Chars>, negative: Negative) -> Token {
    let mut is_float = false;
    let mut number: String = chars
        .by_ref()
        .take_while(|&c| {
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
        Token::Float(string_to_float(&number))
    } else {
        Token::Integer(number.parse().unwrap())
    }
}

fn tokenize_symbol(chars: &mut Peekable<Chars>) -> Token {
    let symbol: String = chars
        .by_ref()
        .take_while(|&c| !c.is_whitespace())
        .collect();
    Token::Symbol(symbol)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => { chars.next(); tokens.push(Token::LeftParen); },
            ')' => { chars.next(); tokens.push(Token::RightParen); },
            '{' => { chars.next(); tokens.push(Token::LeftBrace); },
            '}' => { chars.next(); tokens.push(Token::RightBrace); },
            '[' => { chars.next(); tokens.push(Token::LeftBracket); },
            ']' => { chars.next(); tokens.push(Token::RightBracket); },
            '"' => { tokens.push(tokenize_string(&mut chars)); },
            ':' => { tokens.push(tokenize_keyword(&mut chars)); },
            '-' => {
                chars.next();
                match chars.peek() {
                    Some(&c) if c.is_digit(10) => {
                        tokens.push(tokenize_number(&mut chars, Negative::Yes)); 
                    },
                    _ => { tokens.push(Token::Symbol("-".to_string())); },
                }
            },
            _ if c.is_whitespace() => { chars.next(); },
            _ if c.is_digit(10) => { tokens.push(tokenize_number(&mut chars, Negative::No)); },
            _ => { tokens.push(tokenize_symbol(&mut chars)); }
        }
    }
    tokens
}

