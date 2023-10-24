use forge;

type Result = std::result::Result<(), forge::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let tokens = forge::Tokens::from_str(r#"(server/start {:routes {"/" "Hello Forge"}})"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:3000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "Hello Forge");
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_string_route() -> Result {
    let tokens = forge::Tokens::from_str(
        r#"
        (server/start {:port 4000
                       :routes {"/" "Hello Forge"}})
        "#,
    );
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:4000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "Hello Forge");
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_html_route() -> Result {
    let tokens = forge::Tokens::from_str(
        r#"
        (server/start {:port 8080
                       :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    );
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:8080")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "<ul><li>first</li><li>second</li></ul>");
    Ok(())
}

#[tokio::test]
async fn evaluate_stop() -> Result {
    let tokens = forge::Tokens::from_str("(server/start {:port 9090})");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (environment, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let tokens = forge::Tokens::from_str("(server/stop {:port 9090})");
    let expression = forge::parse(tokens);
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
