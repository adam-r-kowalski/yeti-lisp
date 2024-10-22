#![no_std]
#![forbid(unsafe_code)]
#![feature(iter_array_chunks)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use compiler::effect::{error, Effect};
use compiler::expression::{Call, Environment, Function, Pattern};
use compiler::Expression::{Integer, NativeFunction, Ratio};
use compiler::{array, evaluate_expressions, extract, map, pattern_match, ratio, Expression};
use im::{ordmap, vector, Vector};
use rug;

pub fn truthy(expression: &Expression) -> bool {
    match expression {
        Expression::Nil => false,
        Expression::Bool(false) => false,
        _ => true,
    }
}

pub type Result = core::result::Result<(Environment, Expression), Effect>;

fn function(env: Environment, args: Vector<Expression>) -> core::result::Result<Function, Effect> {
    if let Expression::Array(array) = &args[0] {
        let parameters = array.clone();
        let body = args.skip(1);
        Ok(Function {
            env,
            patterns: vector![Pattern { parameters, body }],
        })
    } else {
        let patterns = args
            .iter()
            .try_fold(Vector::new(), |mut patterns, pattern| {
                let call = extract::call(pattern.clone())?;
                let parameters = extract::array(*call.function)?;
                let body = call.arguments;
                patterns.push_back(Pattern { parameters, body });
                Ok(patterns)
            })?;
        Ok(Function { env, patterns })
    }
}

pub fn environment() -> Environment {
    ordmap! {
        "=".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                  let (env, args) = evaluate_expressions(env, args).await?;
                  Ok((env, Expression::Bool(args[0] == args[1])))
              })
          }
        ),
        "+".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            if args.len() == 0 {
                return Ok((env, Integer(0.into())));
            }
            let (env, args) = evaluate_expressions(env, args).await?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                    (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs + rhs).into())),
                    _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
              })
          }
        ),
        "-".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            let (env, args) = evaluate_expressions(env, args).await?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs - rhs).into())),
                  _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
              })
          }
        ),
        "*".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            if args.len() == 0 {
                return Ok((env, Integer(1.into())));
            }
            let (env, args) = evaluate_expressions(env, args).await?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(Integer((lhs * rhs).into())),
                  (Integer(lhs), Ratio(rhs)) => Ok(ratio((lhs * rhs).into())),
                  (Ratio(lhs), Integer(rhs)) => Ok(ratio((lhs * rhs).into())),
                  (lhs, rhs) => Err(error(&format!("Cannot multiply {} and {}", lhs, rhs))),
                }
            })?;
            Ok((env, result))
              })
          }
        ),
        "/".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            let (env, args) = evaluate_expressions(env, args).await?;
            let (initial, args) = args.split_at(1);
            let result = args.iter().try_fold(initial[0].clone(), |lhs, rhs| {
                match (lhs, rhs) {
                  (Integer(lhs), Integer(rhs)) => Ok(ratio(rug::Rational::from((lhs, rhs)))),
                  _ => Err(error("Expected integer argument")),
                }
            })?;
            Ok((env, result))
              })
          }
        ),
        "if".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = compiler::evaluate(env, condition).await?;
            compiler::evaluate(env, if truthy(&condition) { then } else { otherwise }).await
              })
          }
        ),
        "def".to_string() => NativeFunction(
          |env, args| {
            Box::pin(async move {
              let (name, value) = (args[0].clone(), args[1].clone());
              let (env, value) = compiler::evaluate(env, value).await?;
              let name = extract::symbol(name)?;
              let mut new_env = env.clone();
              new_env.insert(name, value);
              Ok((new_env, Expression::Nil))
            })
          }
        ),
        "fn".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let f = function(env.clone(), args)?;
                Ok((env, Expression::Function(f)))
              })
            }
        ),
        "defn".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                let (name, args) = args.split_at(1);
                let original_env = env.clone();
                let Function{mut env, patterns} = function(env, args)?;
                env.insert("*self*".to_string(), name[0].clone());
                let f = Function{env, patterns};
                compiler::evaluate(original_env, Expression::Call(Call{
                    function: Box::new(Expression::Symbol("def".to_string())),
                    arguments: vector![name[0].clone(), Expression::Function(f)],
                })).await
              })
          }
        ),
        "eval".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                let (env, arg) = compiler::evaluate(env, args[0].clone()).await?;
                compiler::evaluate(env, arg).await
              })
          }
        ),
        "read-string".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                  let (env, arg) = compiler::evaluate(env, args[0].clone()).await?;
                  let s = extract::string(arg)?;
                  let tokens = compiler::tokenize(&s);
                  let (tokens, expression) = compiler::parse(&tokens);
                  if tokens.len() > 0 {
                      return Err(error("Could not parse expression"));
                  }
                  Ok((env, expression))
              })
          }
        ),
        "assert".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            let (env, arg) = compiler::evaluate(env, args[0].clone()).await?;
            if truthy(&arg) {
                Ok((env, Expression::Nil))
            } else {
                Err(error("Assertion failed"))
            }
              })
          }
        ),
        "let".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                let original_env = env.clone();
                let (bindings, body) = args.split_at(1);
                let bindings = extract::array(bindings[0].clone())?;
                let mut let_env = env;
                for [pattern, value] in bindings.iter().array_chunks() {
                    let (e, value) = compiler::evaluate(let_env, value.clone()).await?;
                    let_env = pattern_match(e, pattern.clone(), value)?;
                }
                let (_, values) = compiler::evaluate_expressions(let_env, body).await?;
                Ok((original_env, values.last().unwrap_or(&Expression::Nil).clone()))
              })
          }
        ),
        "for".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                let (bindings, body) = args.split_at(1);
                let bindings = extract::array(bindings[0].clone())?;
                let pattern = bindings[0].clone();
                let (env, values) = compiler::evaluate(env, bindings[1].clone()).await?;
                let values = extract::array(values)?;
                let futures: Vec<_> = values
                    .iter()
                    .map(|value| {
                        let body = body.clone();
                        let pattern = pattern.clone();
                        let env = env.clone();
                        async move {
                            let env = pattern_match(env, pattern, value.clone())?;
                            let (_, value) = compiler::evaluate_expressions(env.clone(), body).await?;
                            Ok(value.last().unwrap_or(&Expression::Nil).clone())
                        }
                    })
                    .collect();
                let results = futures::future::join_all(futures).await;
                let mut values = vector![];
                for result in results {
                    match result {
                        Ok(v) => values.push_back(v),
                        Err(e) => return Err(e),
                    }
                }
                Ok((env, Expression::Array(values)))
              })
          }
        ),
        "str".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
            let (env, args) = evaluate_expressions(env, args).await?;
            let result = args.iter().fold(String::new(), |mut result, arg| {
                if let Expression::String(s) = arg {
                    result.push_str(&s);
                } else {
                    result.push_str(&format!("{}", arg));
                }
                result
            });
            Ok((env, Expression::String(result)))
              })
          }
        ),
        "bound?".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let name = extract::symbol(args[0].clone())?;
                let result = env.contains_key(&name);
                Ok((env, Expression::Bool(result)))
              })
            }
        ),
        "do".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let (env, args) = evaluate_expressions(env, args).await?;
                Ok((env, args.last().unwrap_or(&Expression::Nil).clone()))
              })
            }
        ),
        "->".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let (initial, args) = args.split_at(1);
                let (env, mut result) = compiler::evaluate(env, initial[0].clone()).await?;
                let mut env = env;
                for expression in args.iter() {
                  match expression {
                    Expression::Call(Call{function, arguments}) => {
                      let mut new_arguments = vector![result];
                      new_arguments.append(arguments.clone());
                      let (new_env, value) = compiler::evaluate(env, Expression::Call(Call{
                        function: function.clone(),
                        arguments: new_arguments,
                      })).await?;
                      env = new_env;
                      result = value;
                    }
                    _ => {
                      let (new_env, value) = compiler::evaluate(env, Expression::Call(Call{
                        function: Box::new(expression.clone()),
                        arguments: vector![result],
                      })).await?;
                      env = new_env;
                      result = value;
                    }
                  }
                }
                Ok((env, result))
              })
            }
        ),
        "when".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let (condition, body) = args.split_at(1);
                let (env, condition) = compiler::evaluate(env, condition[0].clone()).await?;
                if truthy(&condition) {
                    compiler::evaluate(env, Expression::Call(Call{
                        function: Box::new(Expression::Symbol("do".to_string())),
                        arguments: body,
                    })).await
                } else {
                    Ok((env, Expression::Nil))
                }
              })
            }
        ),
        "import".to_string() => NativeFunction(
            |env, args| {
              Box::pin(async move {
                let name = extract::symbol(args[0].clone())?;
                let path = format!("{}.yeti", name);
                let (mut env, source) = compiler::evaluate(env, Expression::Call(Call{
                    function: Box::new(Expression::NamespacedSymbol(vec![
                        "io".to_string(),
                        "read-file".to_string()
                    ])),
                    arguments: vector![Expression::String(path)],
                })).await?;
                let source = extract::string(source)?;
                let tokens = compiler::tokenize(&source);
                let expressions = compiler::parse_all(&tokens);
                let mut module = environment();
                module.insert("*name*".to_string(), Expression::String(name.clone()));
                module.insert("io".to_string(), env.get("io").unwrap().clone());
                for expression in expressions.iter() {
                    let (env, _) = compiler::evaluate(module, expression.clone()).await?;
                    module = env;
                }
                env.insert(name, Expression::Module(module));
                Ok((env, Expression::Nil))
              })
            }
        ),
        "inc".to_string() => NativeFunction(
          |env, args| {
              Box::pin(async move {
                compiler::evaluate(env, Expression::Call(Call{
                    function: Box::new(Expression::Symbol("+".to_string())),
                    arguments: vector![args[0].clone(), Expression::Integer(rug::Integer::from(1))],
                })).await
              })
          }
        ),
        "atom".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, value) = compiler::evaluate(env, args[0].clone()).await?;
                    Ok((env, Expression::Atom(compiler::atom::Atom::new(value))))
                })
            }
        ),
        "chan".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    if args.len() == 0 {
                        Ok((env, Expression::Channel(compiler::channel::Channel::new(1))))
                    } else {
                        let buffer_size = extract::integer(args[0].clone())?;
                        let buffer_size = buffer_size.to_usize().ok_or(error("Expected positive integer"))?;
                        Ok((env, Expression::Channel(compiler::channel::Channel::new(buffer_size))))
                    }
                })
            }
        ),
       "put!".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let chan = extract::channel(args[0].clone())?;
                    let value = args[1].clone();
                    if Expression::Nil == value {
                        chan.sender.close();
                    } else {
                        chan.sender.send(value).await.map_err(|_| error("Channel closed"))?;
                    }
                    Ok((env, Expression::Nil))
                })
            }
        ),
        "take!".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let chan = extract::channel(args[0].clone())?;
                    let value = compiler::channel::take(chan).await;
                    Ok((env, value))
                })
            }
        ),
        "close!".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let chan = extract::channel(args[0].clone())?;
                    chan.sender.close();
                    Ok((env, Expression::Nil))
                })
            }
        ),
        "closed?".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let chan = extract::channel(args[0].clone())?;
                    let result = chan.sender.is_closed();
                    Ok((env, Expression::Bool(result)))
                })
            }
        ),
        "reset!".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let atom = extract::atom(args[0].clone())?;
                    let value_to_swap = args[1].clone();
                    let mut value = atom.0.lock().await;
                    *value = value_to_swap;
                    Ok((env, Expression::Nil))
                })
            }
        ),
        "swap!".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let atom = extract::atom(args[0].clone())?;
                    let f = args[1].clone();
                    let mut value = atom.0.lock().await;
                    let (env, new_value) = compiler::evaluate(env, Expression::Call(Call{
                        function: Box::new(f),
                        arguments: vector![value.clone()],
                    })).await?;
                    *value = new_value;
                    Ok((env, Expression::Nil))
                })
            }
        ),
        "range".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let mut start = rug::Integer::from(0);
                    let stop = extract::integer(args[0].clone())?;
                    let step = 1;
                    let mut range = vector![];
                    while start < stop {
                        range.push_back(Expression::Integer(start.clone()));
                        start += step;
                    }
                    Ok((env, Expression::Array(range)))
                })
            }
        ),
        "spawn".to_string() => NativeFunction(
            |env, args| {
                Box::pin(async move {
                    let env_cloned = env.clone();
                    tokio::spawn(async move {
                        let _ = compiler::evaluate_expressions(env, args).await?;
                        Ok::<(), Effect>(())
                    });
                    Ok((env_cloned, Expression::Nil))
                })
            }
        ),
        "assoc".to_string() => NativeFunction(|env, args| Box::pin(map::assoc(env, args))),
        "dissoc".to_string() => NativeFunction(|env, args| Box::pin(map::dissoc(env, args))),
        "merge".to_string() => NativeFunction(|env, args| Box::pin(map::merge(env, args))),
        "get".to_string() => NativeFunction(|env, args| Box::pin(map::get(env, args))),
        "nth".to_string() => NativeFunction(|env, args| Box::pin(array::nth(env, args))),
        "count".to_string() => NativeFunction(|env, args| Box::pin(array::count(env, args)))
    }
}
