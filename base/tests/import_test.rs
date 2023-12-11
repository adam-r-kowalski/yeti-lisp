use base;
use compiler;
use im::ordmap;
use rug::Integer;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn import_module_and_use_def_from_it() -> Result {
    let mut env = base::environment();
    env.insert(
        "io".to_string(),
        compiler::Expression::Module(ordmap! {
            "read-file".to_string() => compiler::Expression::NativeFunction(
                |env, _| Box::pin(async { Ok((env, compiler::Expression::String("(def bar 5)".to_string()))) })
            )
        }),
    );
    let (env, _) = compiler::evaluate_source(env, "(import foo)").await?;
    let (_, actual) = compiler::evaluate_source(env, "foo/bar").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn import_module_with_multiple_definitions() -> Result {
    let mut env = base::environment();
    env.insert(
        "io".to_string(),
        compiler::Expression::Module(ordmap! {
            "read-file".to_string() => compiler::Expression::NativeFunction(
                |env, _| Box::pin(async {Ok((env, compiler::Expression::String(r#"
                    (def bar 5)

                    (def baz 7)
                "#.to_string())))})
            )
        }),
    );
    let (env, _) = compiler::evaluate_source(env, "(import foo)").await?;
    let (env, actual) = compiler::evaluate_source(env, "foo/bar").await?;
    let expected = compiler::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let (_, actual) = compiler::evaluate_source(env, "foo/baz").await?;
    let expected = compiler::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn import_module_with_function() -> Result {
    let mut env = base::environment();
    env.insert(
        "io".to_string(),
        compiler::Expression::Module(ordmap! {
            "read-file".to_string() => compiler::Expression::NativeFunction(
                |env, _| Box::pin(async {Ok((env, compiler::Expression::String(r#"
                    (defn square [x] (* x x))
                "#.to_string())))})
            )
        }),
    );
    let (env, _) = compiler::evaluate_source(env, "(import foo)").await?;
    let (_, actual) = compiler::evaluate_source(env, "(foo/square 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(25));
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn import_multiple_modules() -> Result {
    let mut env = base::environment();
    env.insert(
        "io".to_string(),
        compiler::Expression::Module(ordmap! {
            "read-file".to_string() => compiler::Expression::NativeFunction(
                |env, args| Box::pin(async {
                    let (env, args) = compiler::evaluate_expressions(env, args).await?;
                    let path = compiler::extract::string(args[0].clone())?;
                    match path.as_str() {
                        "foo.yeti" => Ok((env, compiler::Expression::String(r#"
                            (defn square [x] (* x x))
                        "#.to_string()))),
                        "bar.yeti" => Ok((env, compiler::Expression::String(r#"
                            (defn double [x] (* x 2))
                        "#.to_string()))),
                        _ => panic!("Unexpected path {}", path),
                    }
                })
            )
        }),
    );
    let (env, _) = compiler::evaluate_source(env, "(import foo)").await?;
    let (env, _) = compiler::evaluate_source(env, "(import bar)").await?;
    let (env, actual) = compiler::evaluate_source(env, "(foo/square 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(25));
    assert_eq!(actual, expected);
    let (_, actual) = compiler::evaluate_source(env, "(bar/double 5)").await?;
    let expected = compiler::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}
