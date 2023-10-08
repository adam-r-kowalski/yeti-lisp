use std::fmt;
use rug::{Integer, Float};

fn decimal_digits_to_bits(decimal_digits: usize) -> u32 {
    (decimal_digits as f64 * 3.322).ceil() as u32
}

fn bits_to_decimal_digits(bits: u32) -> usize {
    let decimal_digits_float = bits as f64 / 3.322;
    decimal_digits_float.floor() as usize
}

pub fn string_to_float(number: &str) -> Float {
    let digits = number.len() - 1;
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
            '"' => {
                chars.next();
                let string: String = chars
                    .by_ref()
                    .take_while(|&c| c != '"')
                    .collect();
                chars.next();
                tokens.push(Token::String(string));
            },
            ':' => {
                let keyword: String = chars
                    .by_ref()
                    .take_while(|&c| !c.is_whitespace())
                    .collect();
                tokens.push(Token::Keyword(keyword));
            },
            _ if c.is_whitespace() => { chars.next(); },
            _ if c.is_digit(10) => {
                let mut is_float = false;
                let number: String = chars
                    .by_ref()
                    .take_while(|&c| {
                        if c == '.' {
                            is_float = true;
                        }
                        c.is_digit(10) || c == '_' || c == '.'
                    })
                    .filter(|&c| c != '_')
                    .collect();
                if is_float {
                    tokens.push(Token::Float(string_to_float(&number)));
                } else {
                    tokens.push(Token::Integer(number.parse().unwrap()));
                }
            },
            _ => {
                let symbol: String = chars
                    .by_ref()
                    .take_while(|&c| !c.is_whitespace())
                    .collect();
                tokens.push(Token::Symbol(symbol));
            }
        }
    }
    tokens
}

