extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate;
use crate::evaluate_expressions;
use crate::expression::{Call, Environment};
use crate::html::html_string_to_expression;
use crate::Expression::{self, NativeFunction};
use crate::NativeType;
use crate::{extract, html};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use axum::http::Request;
use axum::response::{Html, IntoResponse, Json, Redirect};
use axum::routing::{delete, get, post, put};
use axum::Router;
use core::future::Future;
use core::net::IpAddr;
use core::net::Ipv4Addr;
use core::net::SocketAddr;
use hyper::header::CONTENT_TYPE;
use hyper::Body;
use im::{ordmap, vector, OrdMap};
use reqwest::{RequestBuilder, Response};
use rug::Integer;
use tokio::sync::broadcast;

struct Server {
    tx: broadcast::Sender<()>,
}

type Result<T> = core::result::Result<T, Effect>;

async fn parse_form_data(req: Request<Body>) -> Result<OrdMap<Expression, Expression>> {
    let full_body = hyper::body::to_bytes(req.into_body())
        .await
        .map_err(|_| error("Failed to read body"))?;
    let form_data: BTreeMap<String, String> = serde_qs::from_bytes(&full_body).map_err(|_| {
        error(&format!(
            "Failed to parse form data: {}",
            String::from_utf8_lossy(&full_body)
        ))
    })?;
    let form_data = form_data.iter().fold(OrdMap::new(), |mut m, (k, v)| {
        m.insert(
            Expression::Keyword(format!(":{}", k)),
            Expression::String(v.clone()),
        );
        m
    });
    Ok(form_data)
}

async fn request_map(route_path: &str, req: Request<Body>) -> Result<Expression> {
    let content_type = req
        .headers()
        .get(CONTENT_TYPE)
        .map(|v| v.to_str().unwrap_or(""));
    let method = req.method().to_string();
    let path_and_query = req.uri().path_and_query().unwrap();
    let actual_path = path_and_query.path().to_string();
    let headers = req.headers().iter().fold(OrdMap::new(), |mut m, (k, v)| {
        m.insert(
            Expression::Keyword(format!(":{}", k.to_string())),
            Expression::String(v.to_str().unwrap().to_string()),
        );
        m
    });
    let query = path_and_query.query().unwrap_or("");
    let query: BTreeMap<String, String> = serde_qs::from_str(query).unwrap();
    let query = query.iter().fold(OrdMap::new(), |mut m, (k, v)| {
        m.insert(
            Expression::Keyword(format!(":{}", k.to_string())),
            Expression::String(v.to_string()),
        );
        m
    });
    let params = route_path
        .split('/')
        .zip(actual_path.split('/'))
        .filter(|(a, _)| a.starts_with(':'))
        .fold(OrdMap::new(), |mut m, (a, b)| {
            m.insert(
                Expression::Keyword(a.to_string()),
                Expression::String(b.to_string()),
            );
            m
        });
    let mut map = ordmap![
        Expression::Keyword(":method".to_string()) =>
            Expression::String(method),
        Expression::Keyword(":path".to_string()) =>
            Expression::String(actual_path),
        Expression::Keyword(":headers".to_string()) =>
            Expression::Map(headers)
    ];
    match content_type {
        Some("application/x-www-form-urlencoded") => {
            let form = parse_form_data(req).await.unwrap_or_else(|_| OrdMap::new());
            if !form.is_empty() {
                map.insert(
                    Expression::Keyword(":form".to_string()),
                    Expression::Map(form),
                );
            }
        }
        Some("application/json") => {
            let body_bytes = hyper::body::to_bytes(req.into_body())
                .await
                .map_err(|_| error("Failed to read body"))?;
            let json_data = serde_json::from_slice::<Expression>(&body_bytes)
                .map_err(|_| error("Failed to parse JSON body"))?;
            if let Expression::Map(json_map) = json_data {
                if !json_map.is_empty() {
                    map.insert(
                        Expression::Keyword(":json".to_string()),
                        Expression::Map(json_map),
                    );
                }
            }
        }
        _ => {}
    }
    if query.len() > 0 {
        map.insert(
            Expression::Keyword(":query".to_string()),
            Expression::Map(query),
        );
    }
    if params.len() > 0 {
        map.insert(
            Expression::Keyword(":params".to_string()),
            Expression::Map(params),
        );
    }
    Ok(Expression::Map(map))
}

fn create_handler(expression: Expression) -> impl Future<Output = impl IntoResponse> {
    async move {
        match expression {
            Expression::String(text) => text.into_response(),
            Expression::Array(_) => {
                let mut string = String::new();
                html::build_string(expression, &mut string).unwrap();
                Html(string).into_response()
            }
            Expression::Map(ref m) => {
                if let Some(redirect) = m.get(&Expression::Keyword(":redirect".to_string())) {
                    let url = extract::string(redirect.clone()).unwrap();
                    Redirect::permanent(&url).into_response()
                } else {
                    Json(expression).into_response()
                }
            }
            _ => unimplemented!(),
        }
    }
}

async fn encode_response(response: Response) -> Result<Expression> {
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

fn extend_builder(mut builder: RequestBuilder, params: OrdMap<Expression, Expression>) -> RequestBuilder {
        if let Some(e) = params.get(&Expression::Keyword(":form".to_string())) {
            builder = builder.form(e);
        }
        if let Some(e) = params.get(&Expression::Keyword(":json".to_string())) {
            builder = builder.json(e);
        }
        if let Some(e) = params.get(&Expression::Keyword(":query".to_string())) {
            builder = builder.query(e);
        }
        if let Some(e) = params.get(&Expression::Keyword(":headers".to_string())) {
            if let Expression::Map(headers) = e {
                for (key, value) in headers.iter() {
                    let keyword = extract::keyword(key.clone()).unwrap_or_default();
                    let keyword = &keyword[1..];
                    let value = extract::string(value.clone()).unwrap_or_default();
                    builder = builder.header(keyword, value);
                }
            }
        }
        builder
}

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("http".to_string()),
        "request".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = evaluate_expressions(env, args).await?;
                    let map = extract::map(args[0].clone())?;
                    let url = extract::string(extract::key(map.clone(), ":url")?)?;
                    let default_method = Expression::Keyword(":get".to_string());
                    let method = map.get(&Expression::Keyword(":method".to_string())).unwrap_or(&default_method);
                    let method = extract::keyword(method.clone())?;
                    let client = reqwest::Client::new();
                    let builder = match &method[..] {
                        ":post" => client.post(url),
                        _ => client.get(url)
                    };
                    let builder = extend_builder(builder, map);
                    let response = builder
                        .send()
                        .await
                        .map_err(|_| error("Could not make get request"))?;
                    let response = encode_response(response).await?;
                    Ok((env, response))
                })
            }
        ),
        "server".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, arg) = crate::evaluate(env, args[0].clone()).await?;
                    let m = extract::map(arg)?;
                    let port_expr = m.get(&Expression::Keyword(":port".to_string()));
                    let port = match port_expr {
                        Some(Expression::Integer(i)) => i
                            .to_u16()
                            .ok_or_else(|| error("Port number out of range"))?,
                        None => 3000,
                        _ => return Err(error("Expected integer for :port")),
                    };
                    let mut app = Router::new();
                    if let Some(routes) = m.get(&Expression::Keyword(":routes".to_string())) {
                        let m = extract::map(routes.clone())?;
                        for (k, v) in m.iter() {
                            let path = extract::string(k.clone())?;
                            match v.clone() {
                                Expression::Function(patterns) => {
                                    let env = env.clone();
                                    let cloned_path = path.clone();
                                    let handler = async move |req: Request<Body>| {
                                        let (_, expr) = evaluate(
                                            env,
                                            Expression::Call(Call {
                                                function: Box::new(Expression::Function(patterns.clone())),
                                                arguments: vector![request_map(&cloned_path, req).await.unwrap()],
                                            }),
                                        ).await
                                        .unwrap();
                                        create_handler(expr).await
                                    };
                                    app = app.route(&path, get(handler.clone()));
                                    app = app.route(&path, post(handler.clone()));
                                    app = app.route(&path, delete(handler.clone()));
                                    app = app.route(&path, put(handler));
                                }
                                _ => {
                                    let v = v.clone();
                                    let handler = async move |_req: Request<Body>| {
                                        create_handler(v).await
                                    };
                                    app = app.route(&path, get(handler.clone()));
                                    app = app.route(&path, post(handler.clone()));
                                    app = app.route(&path, delete(handler.clone()));
                                    app = app.route(&path, put(handler));
                                }
                            }
                        }
                    }
                    let (tx, mut rx) = broadcast::channel(1);
                    tokio::spawn(async move {
                        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
                        axum::Server::bind(&socket)
                            .serve(app.into_make_service())
                            .with_graceful_shutdown(async {
                                rx.recv().await.ok();
                            })
                            .await
                            .unwrap();
                    });
                    Ok((
                        env,
                        Expression::NativeType(NativeType::new(Server { tx }, "server".to_string())),
                    ))
                })
            }
        ),
        "server-stop".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, arg) = crate::evaluate(env, args[0].clone()).await?;
                    let server = extract::native_type(arg)?;
                    let server = server.value.lock().await;
                    let server = server
                        .downcast_ref::<Server>()
                        .ok_or_else(|| error("Expected server"))?;
                    server.tx.send(()).unwrap();
                    Ok((env, Expression::Nil))
                })
            }
        )
    }
}
