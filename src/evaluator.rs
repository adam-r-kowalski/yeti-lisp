use im::Vector;

use crate::expression::Environment;
use crate::Expression;

fn evaluate_symbol(environment: Environment, symbol: String) -> (Environment, Expression) {
    if let Some(e) = environment.get(&symbol) {
        (environment.clone(), e.clone())
    } else {
        panic!("Symbol {} not found in environment", symbol)
    }
}

fn evaluate_call(
    environment: Environment,
    function: Expression,
    arguments: Vector<Expression>,
) -> (Environment, Expression) {
    let (environment, function) = evaluate(environment.clone(), function);
    match function {
        Expression::IntrinsicFunction(f) => f(environment, arguments),
        _ => (
            environment,
            Expression::Call {
                function: Box::new(function),
                arguments,
            },
        ),
    }
}

pub fn evaluate(environment: Environment, expression: Expression) -> (Environment, Expression) {
    match expression {
        Expression::Symbol(s) => evaluate_symbol(environment, s),
        Expression::Call {
            function,
            arguments,
        } => evaluate_call(environment, *function, arguments),
        Expression::Array(a) => {
            let (environment, a) = evaluate_expressions(environment, a);
            (environment, Expression::Array(a))
        }
        Expression::Map(m) => {
            let (environment, m) = m.into_iter().fold(
                (environment, im::HashMap::new()),
                |(environment, mut m), (k, v)| {
                    let (environment, k) = evaluate(environment, k);
                    let (environment, v) = evaluate(environment, v);
                    m.insert(k, v);
                    (environment, m)
                },
            );
            (environment, Expression::Map(m))
        }
        e => (environment, e),
    }
}

pub fn evaluate_expressions(
    environment: Environment,
    arguments: Vector<Expression>,
) -> (Environment, Vector<Expression>) {
    arguments.into_iter().fold(
        (environment, Vector::new()),
        |(environment, mut arguments), argument| {
            let (environment, argument) = evaluate(environment, argument);
            arguments.push_back(argument);
            (environment, arguments)
        },
    )
}
