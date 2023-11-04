extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate;
use crate::expression::{Call, Environment};
use crate::{extract, html, Expression};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use axum::http::Request;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use core::net::IpAddr;
use core::net::Ipv4Addr;
use core::net::SocketAddr;
use hyper::Body;
use im::{ordmap, vector, OrdMap, Vector};
use serde_qs;
use tokio::sync::broadcast;

type Result<T> = core::result::Result<T, Effect>;

fn request_map(route_path: &str, req: Request<Body>) -> Expression {
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
    let query_parameters: BTreeMap<String, String> = serde_qs::from_str(query).unwrap();
    let query_parameters = query_parameters
        .iter()
        .fold(OrdMap::new(), |mut m, (k, v)| {
            m.insert(
                Expression::Keyword(format!(":{}", k.to_string())),
                Expression::String(v.to_string()),
            );
            m
        });
    let url_parmaeters = route_path
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
    Expression::Map(ordmap![
        Expression::Keyword(":method".to_string()) =>
            Expression::String(method),
        Expression::Keyword(":path".to_string()) =>
            Expression::String(actual_path),
        Expression::Keyword(":headers".to_string()) =>
            Expression::Map(headers),
        Expression::Keyword(":query-parameters".to_string()) =>
            Expression::Map(query_parameters),
        Expression::Keyword(":url-parameters".to_string()) =>
            Expression::Map(url_parmaeters)
    ])
}

pub fn start(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, arg) = crate::evaluate(env, args[0].clone())?;
    let m = extract::map(arg)?;
    let port_expr = m.get(&Expression::Keyword(":port".to_string()));
    let port = match port_expr {
        Some(Expression::Integer(i)) => i
            .to_u16()
            .ok_or_else(|| error("Port number out of range"))?,
        None => 3000,
        _ => return Err(error("Expected integer for :port")),
    };
    if let Some(tx) = env.servers.lock().get(&port) {
        tx.send(()).unwrap();
    }
    let mut app = Router::new();
    if let Some(routes) = m.get(&Expression::Keyword(":routes".to_string())) {
        let m = extract::map(routes.clone())?;
        for (k, v) in m.iter() {
            let path = extract::string(k.clone())?;
            match v.clone() {
                Expression::String(text) => app = app.route(&path, get(|| async { text })),
                Expression::Array(_) => {
                    let mut string = String::new();
                    html::build_string(v.clone(), &mut string)?;
                    app = app.route(&path, get(|| async { Html(string) }))
                }
                Expression::Function(patterns) => {
                    let env = env.clone();
                    app = app.route(
                        &path.clone(),
                        get(async move |req: Request<Body>| {
                            let (_, expr) = evaluate(
                                env,
                                Expression::Call(Call {
                                    function: Box::new(Expression::Function(patterns.clone())),
                                    arguments: vector![request_map(&path, req)],
                                }),
                            )
                            .unwrap();
                            let mut string = String::new();
                            html::build_string(expr, &mut string).unwrap();
                            Html(string)
                        }),
                    )
                }
                _ => return Err(error("Expected string for route")),
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
    {
        let mut servers = env.servers.lock();
        servers.insert(port, tx.clone());
    }
    Ok((env, Expression::Nil))
}

pub fn shutdown(env: Environment, args: Vector<Expression>) -> Result<(Environment, Expression)> {
    let (env, arg) = crate::evaluate(env, args[0].clone())?;
    let m = extract::map(arg)?;
    let port_expr = m.get(&Expression::Keyword(":port".to_string()));
    let port = match port_expr {
        Some(Expression::Integer(i)) => i
            .to_u16()
            .ok_or_else(|| error("Port number out of range"))?,
        _ => return Err(error("Expected integer for :port")),
    };
    {
        let mut servers = env.servers.lock();
        if let Some(tx) = servers.get(&port) {
            tx.send(()).unwrap();
            servers.remove(&port);
        }
    }
    Ok((env, Expression::Nil))
}
