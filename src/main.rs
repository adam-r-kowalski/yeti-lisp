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
async fn main() -> core::result::Result<(), compiler::effect::Effect> {
    let mut env = repl_environment();
    loop {
        let expressions = repl::read().await?;
        match repl::evaluate(env.clone(), &expressions).await {
            Ok((next_env, expression)) => {
                repl::print(expression).await?;
                env = next_env;
            }
            Err(effect) => repl::print_effect(effect).await?,
        }
    }
}
