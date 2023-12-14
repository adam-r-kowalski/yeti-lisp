#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use compiler;
use compiler::effect::error;
use im::ordmap;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};

type Result<T> = core::result::Result<T, compiler::effect::Effect>;

async fn read_from_stdin() -> String {
    let delimiters = ordmap! {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '"' => '"'
    };
    let mut input = String::new();
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();
    let mut stack = Vec::new();
    while let Ok(Some(l)) = reader.next_line().await {
        if !input.is_empty() {
            input.push('\n');
        }
        input.push_str(&l);
        for char in l.chars() {
            match char {
                '(' | '[' | '{' | '"' => {
                    if stack.last() == Some(&char) && char == '"' {
                        stack.pop();
                    } else {
                        stack.push(char);
                    }
                }
                ')' | ']' | '}' => {
                    if let Some(&opening) = stack.last() {
                        if delimiters.get(&opening) == Some(&char) {
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        let next = input.chars().next().unwrap_or('\0');
        if stack.is_empty() || !delimiters.contains_key(&next) {
            break;
        }
    }
    input
}

pub const BLUE: &str = "\x1b[38;2;58;102;167m";
pub const RED: &str = "\x1b[38;2;211;47;17m";
pub const RESET: &str = "\x1b[0m";

pub async fn read() -> Result<Vec<compiler::Expression>> {
    let mut stdout = io::stdout();
    stdout
        .write_all(format!("{}⛰  {}", BLUE, RESET).as_bytes())
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    stdout
        .flush()
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    let input = read_from_stdin().await;
    let tokens = compiler::tokenize(&input);
    let expressions = compiler::parse_all(&tokens);
    Ok(expressions)
}

pub async fn evaluate(
    env: compiler::Environment,
    exprs: &[compiler::Expression],
) -> Result<(compiler::Environment, compiler::Expression)> {
    let mut env = env;
    let mut result = compiler::Expression::Nil;
    for expr in exprs {
        let (new_env, new_result) = compiler::evaluate(env, expr.clone()).await?;
        env = new_env;
        result = new_result;
    }
    Ok((env, result))
}

pub async fn print(expression: compiler::Expression) -> Result<()> {
    let mut stdout = io::stdout();
    stdout
        .write_all(format!("{}\n", expression).as_bytes())
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    stdout
        .flush()
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    Ok(())
}

pub async fn print_effect(effect: compiler::effect::Effect) -> Result<()> {
    let mut stdout = io::stdout();
    stdout
        .write_all(format!("{}{}{}\n", RED, effect, RESET).as_bytes())
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    stdout
        .flush()
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    Ok(())
}
