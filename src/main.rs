use crossterm::execute;
use crossterm::style::{
    Color::{self, Reset},
    Colors, Print, SetColors,
};
use im::ordmap;
use std::io::{self, Write};
use yeti;

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

fn read(iterator: &mut StdinIterator) -> io::Result<yeti::Expression> {
    print_with_color(BLUE, "⛰  ");
    let tokens = yeti::Tokens::new(iterator);
    Ok(yeti::parse(tokens))
}

fn print(expression: yeti::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

fn repl_environment() -> yeti::Environment {
    let mut environment = yeti::core::environment();
    environment.insert(
        "*name*".to_string(),
        yeti::Expression::String("repl".to_string()),
    );
    environment.insert(
        "io".to_string(),
        yeti::Expression::Module(ordmap! {
            "read-file".to_string() => yeti::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = yeti::evaluate_expressions(env, args).await?;
                        let path = yeti::extract::string(args[0].clone())?;
                        let contents = tokio::fs::read_to_string(path).await
                            .map_err(|_| yeti::effect::error("Could not read file"))?;
                        Ok((env, yeti::Expression::String(contents)))
                    })
                }
            ),
            "write-file".to_string() => yeti::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = yeti::evaluate_expressions(env, args).await?;
                        let path = yeti::extract::string(args[0].clone())?;
                        let contents = yeti::extract::string(args[1].clone())?;
                        tokio::fs::write(path, contents).await
                            .map_err(|_| yeti::effect::error("Could not write file"))?;
                        Ok((env, yeti::Expression::Nil))
                    })
                }
            ),
            "sleep".to_string() => yeti::Expression::NativeFunction(
                |env, args| {
                    Box::pin(async move {
                        let (env, args) = yeti::evaluate_expressions(env, args).await?;
                        let ms = yeti::extract::integer(args[0].clone())?;
                        let ms = ms.to_u64().ok_or(yeti::effect::error("Could not convert integer to u64"))?;
                        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
                        Ok((env, yeti::Expression::Nil))
                    })
                }
            )
        }),
    );
    environment
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut environment = repl_environment();
    let mut iterator = StdinIterator::new();
    loop {
        let expression = read(&mut iterator)?;
        match yeti::evaluate(environment.clone(), expression).await {
            Ok((next_environment, expression)) => {
                print(expression)?;
                environment = next_environment;
            }
            Err(effect) => print_with_color(RED, &format!("{}\n\n", effect)),
        }
    }
}
