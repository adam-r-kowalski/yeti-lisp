use compiler;
use compiler::Expression::Module;
use crossterm::execute;
use crossterm::style::{
    Color::{self, Reset},
    Colors, Print, SetColors,
};
use html;
use http;
use im::ordmap;
use json;
use std::io::{self, Write};
use toml;

struct StdinIterator {
    buffer: String,
    index: usize,
}

impl StdinIterator {
    fn new() -> Self {
        StdinIterator {
            buffer: String::new(),
            index: 0,
        }
    }
}

impl Iterator for StdinIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len() {
            self.buffer.clear();
            self.index = 0;
            io::stdin().read_line(&mut self.buffer).unwrap();
        }
        let result = self.buffer.chars().nth(self.index);
        self.index += 1;
        result
    }
}

fn print_with_color(color: Color, text: &str) -> () {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetColors(Colors {
            foreground: Some(color),
            background: None
        }),
        Print(text),
    )
    .unwrap();
    execute!(
        stdout,
        SetColors(Colors {
            foreground: Some(Reset),
            background: None
        }),
    )
    .unwrap();
}

const BLUE: Color = Color::Rgb {
    r: 58,
    g: 102,
    b: 167,
};

const RED: Color = Color::Rgb {
    r: 211,
    g: 47,
    b: 47,
};

fn read(iterator: &mut StdinIterator) -> io::Result<compiler::Expression> {
    print_with_color(BLUE, "⛰  ");
    let tokens = compiler::Tokens::new(iterator);
    Ok(compiler::parse(tokens))
}

fn print(expression: compiler::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

fn repl_environment() -> compiler::Environment {
    let mut env = compiler::core::environment();
    env.insert(
        "*name*".to_string(),
        compiler::Expression::String("repl".to_string()),
    );
    env.insert("http".to_string(), Module(http::environment()));
    env.insert("html".to_string(), Module(html::environment()));
    env.insert("json".to_string(), Module(json::environment()));
    env.insert("toml".to_string(), Module(toml::environment()));
    env.insert(
        "io".to_string(),
        compiler::Expression::Module(ordmap! {
            "read-file".to_string() => compiler::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = compiler::evaluate_expressions(env, args).await?;
                        let path = compiler::extract::string(args[0].clone())?;
                        let contents = tokio::fs::read_to_string(path).await
                            .map_err(|_| compiler::effect::error("Could not read file"))?;
                        Ok((env, compiler::Expression::String(contents)))
                    })
                }
            ),
            "write-file".to_string() => compiler::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = compiler::evaluate_expressions(env, args).await?;
                        let path = compiler::extract::string(args[0].clone())?;
                        let contents = compiler::extract::string(args[1].clone())?;
                        tokio::fs::write(path, contents).await
                            .map_err(|_| compiler::effect::error("Could not write file"))?;
                        Ok((env, compiler::Expression::Nil))
                    })
                }
            ),
            "sleep".to_string() => compiler::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = compiler::evaluate_expressions(env, args).await?;
                        let ms = compiler::extract::integer(args[0].clone())?;
                        let ms = ms.to_u64().ok_or(compiler::effect::error("Could not convert integer to u64"))?;
                        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
                        Ok((env, compiler::Expression::Nil))
                    })
                }
            )
        }),
    );
    env
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut environment = repl_environment();
    let mut iterator = StdinIterator::new();
    loop {
        let expression = read(&mut iterator)?;
        match compiler::evaluate(environment.clone(), expression).await {
            Ok((next_environment, expression)) => {
                print(expression)?;
                environment = next_environment;
            }
            Err(effect) => print_with_color(RED, &format!("{}\n\n", effect)),
        }
    }
}
