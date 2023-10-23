extern crate alloc;

use crate::effect::{error, Effect};
use crate::expression::{Environment, Result};
use crate::Expression;
use alloc::format;
use alloc::string::String;
use im::Vector;

fn evaluate_symbol(environment: Environment, symbol: String) -> Result {
    if let Some(e) = environment.get(&symbol) {
        Ok((environment.clone(), e.clone()))
    } else {
        Err(error(&format!(
            "Symbol {} not found in environment",
            symbol
        )))
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
        Expression::NativeFunction(f) => f(environment, arguments),
        Expression::Keyword(k) => {
            let (environment, arguments) = evaluate_expressions(environment, arguments)?;
            match &arguments[0] {
                Expression::Map(m) => {
                    if let Some(v) = m.get(&Expression::Keyword(k)) {
                        Ok((environment, v.clone()))
                    } else if arguments.len() == 2 {
                        Ok((environment, arguments[1].clone()))
                    } else {
                        Ok((environment, Expression::Nil))
                    }
                }
                e => Err(error(&format!("Cannot call keyword {} on {}", k, e))),
            }
        }
        Expression::Map(m) => {
            let (environment, arguments) = evaluate_expressions(environment, arguments)?;
            if let Some(v) = m.get(&arguments[0]) {
                Ok((environment, v.clone()))
            } else if arguments.len() == 2 {
                Ok((environment, arguments[1].clone()))
            } else {
                Ok((environment, Expression::Nil))
            }
        }
        _ => Err(error(&format!("Cannot call {}", function))),
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
) -> core::result::Result<(Environment, Vector<Expression>), Effect> {
    expressions.into_iter().try_fold(
        (environment, Vector::new()),
        |(environment, mut expressions), expression| {
            let (environment, argument) = evaluate(environment, expression)?;
            expressions.push_back(argument);
            Ok((environment, expressions))
        },
    )
}
