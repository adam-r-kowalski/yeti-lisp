#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt};
use im::ordmap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use compiler;
use compiler::effect::error;


type Result<T> = core::result::Result<T, compiler::effect::Effect>;

async fn read_from_stdin() -> Result<String> {
    let delimiters = ordmap!{
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
                },
                ')' | ']' | '}' => {
                    if let Some(&opening) = stack.last() {
                        if delimiters.get(&opening) == Some(&char) {
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
        let next = input.chars().next().ok_or(error("Could not read from stdin"))?;
        if stack.is_empty() || !delimiters.contains_key(&next) {
            break;
        }
    }
    Ok(input)
}

pub const BLUE: &str = "\x1b[38;2;58;102;167m";
pub const RED: &str = "\x1b[38;2;211;47;17m";
pub const RESET: &str = "\x1b[0m";

pub async fn read() -> Result<Vec<compiler::Expression>> {
    let mut stdout = io::stdout();
    stdout.write_all(BLUE.as_bytes()).await.map_err(|_| error("Could not write to stdout"))?;
    stdout.write_all("⛰  ".as_bytes()).await.map_err(|_| error("Could not write to stdout"))?;
    stdout.write_all(RESET.as_bytes()).await.map_err(|_| error("Could not write to stdout"))?;
    stdout.flush().await.map_err(|_| error("Could not write to stdout"))?;
    let input = read_from_stdin().await.map_err(|_| error("Could not read from stdin"))?;
    let tokens = compiler::tokenize(&input);
    let expressions = compiler::parse_all(&tokens);
    Ok(expressions)
}

pub async fn evaluate(
    env: compiler::Environment,
    exprs: &[compiler::Expression]
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
    stdout.write_all(format!("{}\n", expression).as_bytes())
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    stdout.flush().await.map_err(|_| error("Could not write to stdout"))?;
    Ok(())
}

pub async fn print_effect(effect: compiler::effect::Effect) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(RED.as_bytes()).await.map_err(|_| error("Could not write to stdout"))?;
    stdout.write_all(format!("{}\n", effect).as_bytes())
        .await
        .map_err(|_| error("Could not write to stdout"))?;
    stdout.write_all(RESET.as_bytes()).await.map_err(|_| error("Could not write to stdout"))?;
    stdout.flush().await.map_err(|_| error("Could not write to stdout"))?;
    Ok(())
}
