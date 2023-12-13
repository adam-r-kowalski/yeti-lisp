use base;
use compiler;
use compiler::Expression::Module;
use html;
use http;
use io;
use json;
use sql;
use toml;
use yaml;
use repl;
use std::io::Result;

use tokio::io::AsyncBufReadExt;
use std::collections::HashMap;


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

// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut env = repl_environment();
//     let mut iterator = repl::StdinIterator::new();
//     loop {
//         let expression = repl::read(&mut iterator)?;
//         match compiler::evaluate(env.clone(), expression).await {
//             Ok((next_env, expression)) => {
//                 repl::print(expression)?;
//                 env = next_env;
//             }
//             Err(effect) => repl::print_with_color(repl::RED, &format!("{}\n\n", effect)),
//         }
//     }
// }


async fn read_from_stdin() -> String {
    let mut input = String::new();
    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin).lines();
    let delimiters = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('"', '"'),
    ]);
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
        if stack.is_empty() || !delimiters.contains_key(&input.chars().next().unwrap()) {
            break;
        }
    }
    input
}


#[tokio::main]
async fn main() {
    let input = read_from_stdin().await;
    println!("Collected input with balanced delimiters: {}", input);
}
