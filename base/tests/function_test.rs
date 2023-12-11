use base;
use compiler;
use compiler::expression::Call;
use rug::Integer;
use im::vector;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn evaluate_fn() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env.clone(), "(fn [x] (* x 2))").await?;
    let expected = compiler::Expression::Function(compiler::expression::Function {
        env,
        patterns: vector![compiler::expression::Pattern {
            parameters: vector![compiler::Expression::Symbol("x".to_string()),],
            body: vector![compiler::Expression::Call(Call {
                function: Box::new(compiler::Expression::Symbol("*".to_string())),
                arguments: vector![
                    compiler::Expression::Symbol("x".to_string()),
                    compiler::Expression::Integer(Integer::from(2)),
                ],
            })],
        }],
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_call_fn() -> Result {
    let env = base::environment();
    let (_, actual) = compiler::evaluate_source(env, "((fn [x] (* x 2)) 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}


#[tokio::test]
async fn function_definition_and_call() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(env, "(defn double [x] (* x 2))").await?;
    let (env, actual) = compiler::evaluate_source(env, "(double 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    let actual = compiler::evaluate_source(env, "x").await;
    assert!(matches!(actual, Err(compiler::effect::Effect::Error(_))));
    Ok(())
}

#[tokio::test]
async fn closure() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(
        env,
        r#"
         (defn add [x]
          (fn [y] (+ x y)))
        "#,
    )
    .await?;
    let (env, _) = compiler::evaluate_source(env, "(def add-5 (add 5))").await?;
    let (_, actual) = compiler::evaluate_source(env, "(add-5 10)").await?;
    let expected = compiler::Expression::Integer(Integer::from(15));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn recursion_using_recur() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(
        env,
        r#"
         (defn fib
          ([0] 1)
          ([1] 1)
          ([n] (+ (recur (- n 1)) (recur (- n 2)))))
        "#,
    )
    .await?;
    let (env, actual) = compiler::evaluate_source(env, "(fib 0)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 1)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 2)").await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    let (_, actual) = compiler::evaluate_source(env, "(fib 4)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn recursion_using_name() -> Result {
    let env = base::environment();
    let (env, _) = compiler::evaluate_source(
        env,
        r#"
         (defn fib
          ([0] 1)
          ([1] 1)
          ([n] (+ (fib (- n 1)) (fib (- n 2)))))
        "#,
    )
    .await?;
    let (env, actual) = compiler::evaluate_source(env, "(fib 0)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 1)").await?;
    let expected = compiler::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 2)").await?;
    let expected = compiler::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 3)").await?;
    let expected = compiler::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    let (env, actual) = compiler::evaluate_source(env, "(fib 4)").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let (_, actual) = compiler::evaluate_source(env, "(fib 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
    Ok(())
}
