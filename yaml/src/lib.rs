#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use compiler::effect::error;
use compiler::evaluate_expressions;
use compiler::expression::Environment;
use compiler::extract;
use compiler::Expression::{self, NativeFunction};
use alloc::boxed::Box;
use alloc::string::ToString;
use im::ordmap;

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("yaml".to_string()),
        "to-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let yaml = serde_yaml::to_string(&args[0])
                        .map_err(|_| error("Could not convert to yaml"))?;
                    Ok((env, Expression::String(yaml)))
                })
            }
        ),
        "from-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let string = extract::string(args[0].clone())?;
                    let yaml = serde_yaml::from_str::<Expression>(&string)
                        .map_err(|_| error("Could not parse yaml"))?;
                    Ok((env, yaml))
                })
            }
        )
    }
}
