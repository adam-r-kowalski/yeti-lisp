use rug::Integer;

#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(f64),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
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
                    tokens.push(Token::Float(number.parse().unwrap()));
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

