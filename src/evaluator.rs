use im::Vector;

use crate::Expression;

pub type Environment = im::HashMap<String, Expression>;

fn evaluate_symbol(environment: Environment, symbol: String) -> (Environment, Expression) {
    if let Some(e) = environment.get(&symbol) {
        (environment.clone(), e.clone())
    } else {
        (environment, Expression::Symbol(symbol))
    }
}

fn reduce(
    (environment, mut arguments): (Environment, Vector<Expression>),
    argument: Expression,
) -> (Environment, Vector<Expression>) {
    let (environment, argument) = evaluate(environment.clone(), argument);
    arguments.push_back(argument);
    (environment, arguments)
}

fn evaluate_call(
    environment: Environment,
    function: Expression,
    arguments: Vector<Expression>,
) -> (Environment, Expression) {
    let (environment, function) = evaluate(environment.clone(), function);
    let (environment, arguments) = arguments
        .into_iter()
        .fold((environment, Vector::new()), reduce);
    match function {
        Expression::IntrinsicFunction(f) => (environment, f(arguments)),
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
        e => (environment, e),
    }
}
