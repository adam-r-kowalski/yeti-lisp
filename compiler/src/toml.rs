extern crate alloc;

use crate::effect::error;
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::Expression::{self, NativeFunction};
use alloc::boxed::Box;
use alloc::string::ToString;
use im::ordmap;

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("toml".to_string()),
        "to-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let toml = toml::to_string(&args[0])
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
                    let toml = toml::from_str::<Expression>(&string)
                        .map_err(|_| error("Could not parse TOML"))?;
                    Ok((env, toml))
                })
            }
        )
    }
}
