use compiler;
use compiler::expression::Expression::Module;
use toml;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn toml_to_string_for_map() -> Result {
    let mut env = compiler::core::environment();
    env.insert(
        "toml".to_string(),
        Module(toml::environment()),
    );
    let (_, actual) =
        compiler::evaluate_source(env, r#"(toml/to-string {:first "John" :last "Smith"})"#).await?;
    let expected = compiler::Expression::String("first = \"John\"\nlast = \"Smith\"\n".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn toml_to_string_for_map_with_int() -> Result {
    let mut env = compiler::core::environment();
    env.insert(
        "toml".to_string(),
        Module(toml::environment()),
    );
    let (_, actual) =
        compiler::evaluate_source(env, r#"(toml/to-string {:first "John" :age 20})"#).await?;
    let expected = compiler::Expression::String("age = 20\nfirst = \"John\"\n".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn toml_from_string_for_map() -> Result {
    let mut env = compiler::core::environment();
    env.insert(
        "toml".to_string(),
        Module(toml::environment()),
    );
    let (env, actual) = compiler::evaluate_source(
        env,
        r#"(toml/from-string "first = \"John\"\nlast = \"Smith\"\n"#,
    )
    .await?;
    let (_, expected) = compiler::evaluate_source(env, r#"{:first "John" :last "Smith"}"#).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn toml_from_string_for_map_with_int() -> Result {
    let mut env = compiler::core::environment();
    env.insert(
        "toml".to_string(),
        Module(toml::environment()),
    );
    let (env, actual) =
        compiler::evaluate_source(env, r#"(toml/from-string "age = 20\nfirst = \"John\"\n")"#)
            .await?;
    let (_, expected) = compiler::evaluate_source(env, r#"{:first "John" :age 20}"#).await?;
    assert_eq!(actual, expected);
    Ok(())
}
