extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::html::html_string_to_expression;
use crate::Expression::{self, NativeFunction};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use im::ordmap;
use reqwest::Response;
use rug::Integer;

async fn encode_response(response: Response) -> Result<Expression, Effect> {
    let status = Expression::Integer(Integer::from(response.status().as_u16()));
    let headers = response.headers();
    let mut headers_map = ordmap! {};
    for (key, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            headers_map.insert(
                Expression::Keyword(format!(":{}", key)),
                Expression::String(value_str.to_string()),
            );
        }
    }
    let url = Expression::String(response.url().to_string());
    let mut result = ordmap! {
        Expression::Keyword(":status".to_string()) => status,
        Expression::Keyword(":headers".to_string()) => Expression::Map(headers_map),
        Expression::Keyword(":url".to_string()) => url
    };
    let content_type = headers
        .get("content-type")
        .map(|value| value.to_str().unwrap_or(""))
        .unwrap_or("");
    match content_type {
        "application/json" => {
            let json = response
                .json::<Expression>()
                .await
                .map_err(|_| error("Could not get text from response"))?;
            result.insert(Expression::Keyword(":json".to_string()), json);
        }
        t if t.starts_with("text/html") => {
            let text = response
                .text()
                .await
                .map_err(|_| error("Could not get text from response"))?;
            let expression = html_string_to_expression(&text);
            result.insert(Expression::Keyword(":html".to_string()), expression);
        }
        _ => {
            let text = response
                .text()
                .await
                .map_err(|_| error("Could not get text from response"))
                .map(|text| Expression::String(text))?;
            result.insert(Expression::Keyword(":text".to_string()), text);
        }
    }
    Ok(Expression::Map(result))
}

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("http".to_string()),
        "get".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let url = extract::string(args[0].clone())?;
                    let client = reqwest::Client::new();
                    let mut builder = client.get(url);
                    if let Some(e) = args.get(1) {
                        let params = extract::map(e.clone())?;
                        if let Some(e) = params.get(&Expression::Keyword(":query".to_string())) {
                            builder = builder.query(e);
                        }
                    }
                    let response = builder
                        .send()
                        .await
                        .map_err(|_| error("Could not make get request"))?;
                    let response = encode_response(response).await?;
                    Ok((env, response))
                })
            }
        ),
        "post".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let url = extract::string(args[0].clone())?;
                    let params = extract::map(args[1].clone())?;
                    let client = reqwest::Client::new();
                    let mut builder = client.post(url);
                    if let Some(e) = params.get(&Expression::Keyword(":form".to_string())) {
                        builder = builder.form(e);
                    }
                    if let Some(e) = params.get(&Expression::Keyword(":json".to_string())) {
                        builder = builder.json(e);
                    }
                    let response = builder
                        .send()
                        .await
                        .map_err(|_| error("Could not make post request"))?;
                    let response = encode_response(response).await?;
                    Ok((env, response))
                })
            }
        )
    }
}
