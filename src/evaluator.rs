extern crate alloc;

use crate::expression::{Environment, RaisedEffect, Result};
use crate::Expression;
use alloc::format;
use alloc::string::{String, ToString};
use im::{vector, HashMap, Vector};

fn evaluate_symbol(environment: Environment, symbol: String) -> Result {
    if let Some(e) = environment.get(&symbol) {
        Ok((environment.clone(), e.clone()))
    } else {
        Err(RaisedEffect {
            environment,
            effect: "error".to_string(),
            arguments: vector![Expression::String(format!(
                "Symbol {} not found in environment",
                symbol
            ))],
        })
    }
}

fn lookup_key_in_map(
    environment: Environment,
    map: HashMap<Expression, Expression>,
    keyword: String,
) -> Result {
    if let Some(e) = map.get(&Expression::Keyword(keyword.clone())) {
        Ok((environment, e.clone()))
    } else {
        Err(RaisedEffect {
            environment,
            effect: "error".to_string(),
            arguments: vector![Expression::String(format!(
                "Keyword {} not found in map {}",
                keyword,
                Expression::Map(map)
            ))],
        })
    }
}

fn evaluate_call(
    environment: Environment,
    function: Expression,
    arguments: Vector<Expression>,
) -> Result {
    let (environment, function) = evaluate(environment.clone(), function)?;
    match function {
        Expression::Function { parameters, body } => {
            let (environment, arguments) = evaluate_expressions(environment, arguments)?;
            let environment = parameters.into_iter().zip(arguments.into_iter()).fold(
                environment,
                |mut environment, (parameter, argument)| match parameter {
                    Expression::Symbol(s) => {
                        environment.insert(s, argument);
                        environment
                    }
                    _ => panic!("Invalid parameter type"),
                },
            );
            evaluate(environment, *body)
        }
        Expression::IntrinsicFunction(f) => f(environment, arguments),
        Expression::Keyword(k) => {
            let (environment, arguments) = evaluate_expressions(environment, arguments)?;
            match &arguments[0] {
                Expression::Map(m) => lookup_key_in_map(environment, m.clone(), k),
                e => Err(RaisedEffect {
                    environment,
                    effect: "error".to_string(),
                    arguments: vector![Expression::String(format!(
                        "Cannot call keyword {} on {}",
                        k, e
                    ))],
                }),
            }
        }
        Expression::Map(m) => {
            let (environment, arguments) = evaluate_expressions(environment, arguments)?;
            match &arguments[0] {
                Expression::Keyword(k) => lookup_key_in_map(environment, m, k.clone()),
                e => Err(RaisedEffect {
                    environment,
                    effect: "error".to_string(),
                    arguments: vector![Expression::String(format!(
                        "Cannot call map {} on {}",
                        Expression::Map(m),
                        e
                    ))],
                }),
            }
        }
        _ => Err(RaisedEffect {
            environment,
            effect: "error".to_string(),
            arguments: vector![Expression::String(format!("Cannot call {}", function))],
        }),
    }
}

pub fn evaluate(environment: Environment, expression: Expression) -> Result {
    match expression {
        Expression::Symbol(s) => evaluate_symbol(environment, s),
        Expression::Call {
            function,
            arguments,
        } => evaluate_call(environment, *function, arguments),
        Expression::Array(a) => {
            let (environment, a) = evaluate_expressions(environment, a)?;
            Ok((environment, Expression::Array(a)))
        }
        Expression::Map(m) => {
            let (environment, m) = m.into_iter().try_fold(
                (environment, im::HashMap::new()),
                |(environment, mut m), (k, v)| {
                    let (environment, k) = evaluate(environment, k)?;
                    let (environment, v) = evaluate(environment, v)?;
                    m.insert(k, v);
                    Ok((environment, m))
                },
            )?;
            Ok((environment, Expression::Map(m)))
        }
        Expression::Quote(e) => Ok((environment, *e)),
        e => Ok((environment, e)),
    }
}

pub fn evaluate_expressions(
    environment: Environment,
    expressions: Vector<Expression>,
) -> core::result::Result<(Environment, Vector<Expression>), RaisedEffect> {
    expressions.into_iter().try_fold(
        (environment, Vector::new()),
        |(environment, mut expressions), expression| {
            let (environment, argument) = evaluate(environment, expression)?;
            expressions.push_back(argument);
            Ok((environment, expressions))
        },
    )
}
