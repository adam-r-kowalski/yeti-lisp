use forge;
use std::io::{self, Write};

fn read() -> io::Result<forge::Expression> {
    print!("λ ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let tokens = forge::Tokens::new(&buffer);
    Ok(forge::parse(tokens))
}

fn print(expression: forge::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

fn main() -> io::Result<()> {
    let mut environment = forge::core::environment();
    loop {
        let expression = read()?;
        match forge::evaluate(environment.clone(), expression) {
            Ok((next_environment, expression)) => {
                print(expression)?;
                environment = next_environment;
            }
            Err(forge::RaisedEffect {
                environment: _,
                effect,
                arguments,
            }) => {
                println!(
                    "{{:effect {}, :arguments {}}}\n",
                    effect,
                    forge::Expression::Array(arguments)
                )
            }
        }
    }
}
