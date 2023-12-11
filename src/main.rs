use base;
use compiler;
use compiler::Expression::Module;
use crossterm::execute;
use crossterm::style::{
    Color::{self, Reset},
    Colors, Print, SetColors,
};
use html;
use http;
use io;
use json;
use sql;
use std::io::{self as std_io, Write};
use toml;
use yaml;

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
            std_io::stdin().read_line(&mut self.buffer).unwrap();
        }
        let result = self.buffer.chars().nth(self.index);
        self.index += 1;
        result
    }
}

fn print_with_color(color: Color, text: &str) -> () {
    let mut stdout = std_io::stdout();
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

fn read(iterator: &mut StdinIterator) -> std_io::Result<compiler::Expression> {
    print_with_color(BLUE, "⛰  ");
    let tokens = compiler::Tokens::new(iterator);
    Ok(compiler::parse(tokens))
}

fn print(expression: compiler::Expression) -> std_io::Result<()> {
    std_io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

fn repl_environment() -> compiler::Environment {
    let mut env = base::environment();
    env.insert(
        "*name*".to_string(),
        compiler::Expression::String("repl".to_string()),
    );
    env.insert("html".to_string(), Module(html::environment()));
    env.insert("http".to_string(), Module(http::environment()));
    env.insert("io".to_string(), Module(io::environment()));
    env.insert("json".to_string(), Module(json::environment()));
    env.insert("sql".to_string(), Module(sql::environment()));
    env.insert("toml".to_string(), Module(toml::environment()));
    env.insert("yaml".to_string(), Module(yaml::environment()));
    env
}

#[tokio::main]
async fn main() -> std_io::Result<()> {
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
