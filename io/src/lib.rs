#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::ToString;
use compiler::expression::Environment;
use compiler::Expression;
use im::ordmap;

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("io".to_string()),
        "read-file".to_string() => compiler::Expression::NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let path = compiler::extract::string(args[0].clone())?;
                    let contents = tokio::fs::read_to_string(path).await
                        .map_err(|_| compiler::effect::error("Could not read file"))?;
                    Ok((env, compiler::Expression::String(contents)))
                })
            }
        ),
        "write-file".to_string() => compiler::Expression::NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let path = compiler::extract::string(args[0].clone())?;
                    let contents = compiler::extract::string(args[1].clone())?;
                    tokio::fs::write(path, contents).await
                        .map_err(|_| compiler::effect::error("Could not write file"))?;
                    Ok((env, compiler::Expression::Nil))
                })
            }
        ),
        "sleep".to_string() => compiler::Expression::NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let ms = compiler::extract::integer(args[0].clone())?;
                    let ms = ms.to_u64().ok_or(compiler::effect::error("Could not convert integer to u64"))?;
                    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
                    Ok((env, compiler::Expression::Nil))
                })
            }
        )
    }
}
