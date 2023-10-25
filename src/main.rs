use yeti;
use std::io::{self, Write};

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

fn read(iterator: &mut StdinIterator) -> io::Result<yeti::Expression> {
    print!("λ ");
    io::stdout().flush()?;
    let tokens = yeti::Tokens::new(iterator);
    Ok(yeti::parse(tokens))
}

fn print(expression: yeti::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut environment = yeti::core::environment();
    let mut iterator = StdinIterator::new();
    loop {
        let expression = read(&mut iterator)?;
        match yeti::evaluate(environment.clone(), expression) {
            Ok((next_environment, expression)) => {
                print(expression)?;
                environment = next_environment;
            }
            Err(effect) => println!("{:?}\n", effect),
        }
    }
}
