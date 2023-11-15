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
        "*name*".to_string() => Expression::String("http".to_string()),
        "get".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let url = extract::string(args[0].clone())?;
                    let response = reqwest::get(url)
                        .await
                        .map_err(|_| error("Could not make get request"))?
                        .text()
                        .await
                        .map_err(|_| error("Could not get text from response"))
                        .map(|text| Expression::String(text))?;
                    Ok((env, response))
                })
            }
        )
    }
}
