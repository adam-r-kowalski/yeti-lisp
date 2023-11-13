use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn function_definition_and_call() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(defn double [x] (* x 2))")?;
    let (env, actual) = yeti::evaluate_source(env, "(double 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    let actual = yeti::evaluate_source(env, "x");
    assert!(matches!(actual, Err(yeti::effect::Effect::Error(_))));
    Ok(())
}

#[test]
fn multi_line_function() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
         (defn sum-of-squares [x y]
          (def x2 (* x x))
          (def y2 (* y y))
          (+ x2 y2))
        "#,
    )?;
    let (_, actual) = yeti::evaluate_source(env, "(sum-of-squares 5 7)")?;
    let expected = yeti::Expression::Integer(Integer::from(74));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn closure() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
         (defn add [x]
          (fn [y] (+ x y)))
        "#,
    )?;
    let (env, _) = yeti::evaluate_source(env, "(def add-5 (add 5))")?;
    let (_, actual) = yeti::evaluate_source(env, "(add-5 10)")?;
    let expected = yeti::Expression::Integer(Integer::from(15));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn recursion_using_recur() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
         (defn fib
          ([0] 1)
          ([1] 1)
          ([n] (+ (recur (- n 1)) (recur (- n 2)))))
        "#,
    )?;
    let (env, actual) = yeti::evaluate_source(env, "(fib 0)")?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 1)")?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 2)")?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 3)")?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    let (_, actual) = yeti::evaluate_source(env, "(fib 4)")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn recursion_using_name() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
         (defn fib
          ([0] 1)
          ([1] 1)
          ([n] (+ (fib (- n 1)) (fib (- n 2)))))
        "#,
    )?;
    let (env, actual) = yeti::evaluate_source(env, "(fib 0)")?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 1)")?;
    let expected = yeti::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 2)")?;
    let expected = yeti::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 3)")?;
    let expected = yeti::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "(fib 4)")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let (_, actual) = yeti::evaluate_source(env, "(fib 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
    Ok(())
}
