extern crate alloc;

use crate::effect::error;
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::Expression::{self, NativeFunction};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use im::ordmap;
use rug::Integer;

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
                        .map_err(|_| error("Could not make get request"))?;
                    let status = Expression::Integer(Integer::from(response.status().as_u16()));
                    let headers = response.headers();
                    let mut headers_map = ordmap!{};
                    for (key, value) in headers.iter() {
                        if let Ok(value_str) = value.to_str() {
                            headers_map.insert(
                                Expression::Keyword(format!(":{}", key)),
                                Expression::String(value_str.to_string()),
                            );
                        }
                    }
                    let url = Expression::String(response.url().to_string());
                    let text = response
                        .text()
                        .await
                        .map_err(|_| error("Could not get text from response"))
                        .map(|text| Expression::String(text))?;
                    let response = Expression::Map(ordmap!{
                        Expression::Keyword(":status".to_string()) => status,
                        Expression::Keyword(":headers".to_string()) => Expression::Map(headers_map),
                        Expression::Keyword(":url".to_string()) => url,
                        Expression::Keyword(":text".to_string()) => text
                    });
                    Ok((env, response))
                })
            }
        )
    }
}
