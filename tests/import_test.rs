/*
use im::ordmap;
use rug::Integer;
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn import_module_and_use_def_from_it() -> Result {
    let mut env = yeti::core::environment();
    env.insert(
        "io".to_string(),
        yeti::Expression::Module(ordmap! {
            "read-file-sync".to_string() => yeti::Expression::NativeFunction(
                |env, _| Ok((env, yeti::Expression::String("(def bar 5)".to_string())))
            )
        }),
    );
    let (env, _) = yeti::evaluate_source(env, "(import foo)")?;
    let (_, actual) = yeti::evaluate_source(env, "foo/bar")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn import_module_with_multiple_definitions() -> Result {
    let mut env = yeti::core::environment();
    env.insert(
        "io".to_string(),
        yeti::Expression::Module(ordmap! {
            "read-file-sync".to_string() => yeti::Expression::NativeFunction(
                |env, _| Ok((env, yeti::Expression::String(r#"
                    (def bar 5)

                    (def baz 7)
                "#.to_string())))
            )
        }),
    );
    let (env, _) = yeti::evaluate_source(env, "(import foo)")?;
    let (env, actual) = yeti::evaluate_source(env, "foo/bar")?;
    let expected = yeti::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    let (_, actual) = yeti::evaluate_source(env, "foo/baz")?;
    let expected = yeti::Expression::Integer(Integer::from(7));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn import_module_with_function() -> Result {
    let mut env = yeti::core::environment();
    env.insert(
        "io".to_string(),
        yeti::Expression::Module(ordmap! {
            "read-file-sync".to_string() => yeti::Expression::NativeFunction(
                |env, _| Ok((env, yeti::Expression::String(r#"
                    (defn square [x] (* x x))
                "#.to_string())))
            )
        }),
    );
    let (env, _) = yeti::evaluate_source(env, "(import foo)")?;
    let (_, actual) = yeti::evaluate_source(env, "(foo/square 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(25));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn import_multiple_modules() -> Result {
    let mut env = yeti::core::environment();
    env.insert(
        "io".to_string(),
        yeti::Expression::Module(ordmap! {
            "read-file-sync".to_string() => yeti::Expression::NativeFunction(
                |env, args| {
                    let (env, args) = yeti::evaluate_expressions(env, args)?;
                    let path = yeti::extract::string(args[0].clone())?;
                    match path.as_str() {
                        "foo.yeti" => Ok((env, yeti::Expression::String(r#"
                            (defn square [x] (* x x))
                        "#.to_string()))),
                        "bar.yeti" => Ok((env, yeti::Expression::String(r#"
                            (defn double [x] (* x 2))
                        "#.to_string()))),
                        _ => panic!("Unexpected path {}", path),
                    }
                }
            )
        }),
    );
    let (env, _) = yeti::evaluate_source(env, "(import foo)")?;
    let (env, _) = yeti::evaluate_source(env, "(import bar)")?;
    let (env, actual) = yeti::evaluate_source(env, "(foo/square 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(25));
    assert_eq!(actual, expected);
    let (_, actual) = yeti::evaluate_source(env, "(bar/double 5)")?;
    let expected = yeti::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}
*/
