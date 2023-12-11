use compiler;
use compiler::Expression::Module;
use yaml;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn yaml_to_string_for_map() -> Result {
    let mut env = compiler::core::environment();
    env.insert("yaml".to_string(), Module(yaml::environment()));
    let (_, actual) =
        compiler::evaluate_source(env, r#"(yaml/to-string {:first "John" :last "Smith"})"#).await?;
    let expected = compiler::Expression::String("first: John\nlast: Smith\n".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn yaml_to_string_for_map_with_int() -> Result {
    let mut env = compiler::core::environment();
    env.insert("yaml".to_string(), Module(yaml::environment()));
    let (_, actual) =
        compiler::evaluate_source(env, r#"(yaml/to-string {:first "John" :age 20})"#).await?;
    let expected = compiler::Expression::String("age: 20\nfirst: John\n".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn yaml_from_string_for_map() -> Result {
    let mut env = compiler::core::environment();
    env.insert("yaml".to_string(), Module(yaml::environment()));
    let (env, actual) =
        compiler::evaluate_source(env, r#"(yaml/from-string "first: John\nlast: Smith\n")"#)
            .await?;
    let (_, expected) = compiler::evaluate_source(env, r#"{:first "John" :last "Smith"}"#).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn yaml_from_string_for_map_with_int() -> Result {
    let mut env = compiler::core::environment();
    env.insert("yaml".to_string(), Module(yaml::environment()));
    let (env, actual) =
        compiler::evaluate_source(env, r#"(yaml/from-string "first: John\nage: 20\n")"#).await?;
    let (_, expected) = compiler::evaluate_source(env, r#"{:first "John" :age 20}"#).await?;
    assert_eq!(actual, expected);
    Ok(())
}
