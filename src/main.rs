use std::io::{self, Write};
use tao;

fn read() -> io::Result<tao::Expression> {
    print!("λ ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let tokens = tao::tokenize(&buffer);
    Ok(tao::parse(tokens))
}

fn print(expression: tao::Expression) -> io::Result<()> {
    io::stdout().write_all(format!("{}", expression).as_bytes())?;
    println!("\n");
    Ok(())
}

fn main() -> io::Result<()> {
    let mut environment = tao::core::environment();
    loop {
        let expression = read()?;
        match tao::evaluate(environment.clone(), expression) {
            Ok((next_environment, expression)) => {
                print(expression)?;
                environment = next_environment;
            }
            Err(tao::RaisedEffect {
                environment: _,
                effect,
                arguments,
            }) => {
                println!(
                    "{{:effect {}, :arguments {}}}\n",
                    effect,
                    tao::Expression::Array(arguments)
                )
            }
        }
    }
}
