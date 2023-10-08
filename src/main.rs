use std::io::{self, Write};
use tao;

fn main() -> io::Result<()> {
    loop {
        print!("λ ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let tokens = tao::tokenize(&buffer);
        io::stdout().write_all(format!("{:?}", tokens).as_bytes())?;
        println!("");
    }
}
