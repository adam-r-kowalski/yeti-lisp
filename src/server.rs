extern crate alloc;

use core::net::IpAddr;
use core::net::Ipv4Addr;
use core::net::SocketAddr;

use crate::build_html_string;
use crate::effect::{error, Effect};
use crate::expression::Environment;
use crate::extract;
use crate::Expression;
use alloc::string::{String, ToString};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use im::Vector;
use tokio::sync::broadcast;

type Result<T> = core::result::Result<T, Effect>;

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
                    build_html_string(v.clone(), &mut string)?;
                    app = app.route(&path, get(|| async { Html(string) }))
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