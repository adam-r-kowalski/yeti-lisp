#![forbid(unsafe_code)]

use compiler;
use crossterm::execute;
use crossterm::style::{
    Color::{self, Reset},
    Colors, Print, SetColors,
};
use std::io::{self as io, Write};

pub struct StdinIterator {
    buffer: String,
    index: usize,
}

impl StdinIterator {
    pub fn new() -> Self {
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

pub fn print_with_color(color: Color, text: &str) -> () {
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

pub const BLUE: Color = Color::Rgb {
    r: 58,
    g: 102,
    b: 167,
};

pub const RED: Color = Color::Rgb {
    r: 211,
    g: 47,
    b: 47,
};

pub fn read(iterator: &mut StdinIterator) -> io::Result<compiler::Expression> {
    print_with_color(BLUE, "⛰  ");
    let tokens = compiler::Tokens::new(iterator);
    Ok(compiler::parse(tokens))
}

pub fn print(expression: compiler::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}
