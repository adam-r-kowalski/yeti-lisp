#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::ToString;
use compiler::effect::error;
use compiler::evaluate_expressions;
use compiler::expression::Environment;
use compiler::extract;
use compiler::Expression::{self, NativeFunction};
use im::ordmap;

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("toml".to_string()),
        "to-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let toml = toml_lib::to_string(&args[0])
                        .map_err(|_| error("Could not convert to TOML"))?;
                    Ok((env, Expression::String(toml)))
                })
            }
        ),
        "from-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let string = extract::string(args[0].clone())?;
                    let toml = toml_lib::from_str::<Expression>(&string)
                        .map_err(|_| error("Could not parse TOML"))?;
                    Ok((env, toml))
                })
            }
        )
    }
}
