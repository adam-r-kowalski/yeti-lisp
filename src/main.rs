use std::io::{self, Write};
use tao;

fn main() -> io::Result<()> {
    loop {
        print!("λ ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let tokens = tao::tokenize(&buffer);
        let expression = tao::parse(tokens);
        io::stdout().write_all(format!("{:?}", expression).as_bytes())?;
        println!("");
    }
}
