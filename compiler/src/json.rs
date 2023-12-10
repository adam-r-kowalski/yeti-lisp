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
        "*name*".to_string() => Expression::String("json".to_string()),
        "to-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let json = serde_json::to_string_pretty(&args[0])
                        .map_err(|_| error("Could not convert to json"))?;
                    Ok((env, Expression::String(json)))
                })
            }
        ),
        "from-string".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let string = extract::string(args[0].clone())?;
                    let json = serde_json::from_str::<Expression>(&string)
                        .map_err(|_| error("Could not parse json"))?;
                    Ok((env, json))
                })
            }
        )
    }
}
