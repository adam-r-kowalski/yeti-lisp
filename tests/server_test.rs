use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(server/start {:routes {"/" "Hello Forge"}})"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
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
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 4000
                       :routes {"/" "Hello Forge"}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
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
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 8080
                       :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
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
async fn evaluate_server_with_function_route() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 8080
                       :routes {"/" (fn [req] [:ul [:li "first"] [:li "second"]])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
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
async fn evaluate_server_show_request_as_str() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 10080
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:10080")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10080"}, :method "GET", :path "/", :query-parameters {}, :url-parameters {}}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_server_query_parameters() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 10090
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:10090?foo=bar&baz=qux")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10090"}, :method "GET", :path "/", :query-parameters {:baz "qux", :foo "bar"}, :url-parameters {}}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_server_url_parameters() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 10070
                       :routes {"/hello/:name" (fn [req] [:p (str req)])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let body = reqwest::get("http://localhost:10070/hello/joe")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10070"}, :method "GET", :path "/hello/joe", :query-parameters {}, :url-parameters {:name "joe"}}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_stop() -> Result {
    let tokens = yeti::Tokens::from_str("(server/start {:port 9090})");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (environment, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    let tokens = yeti::Tokens::from_str("(server/stop {:port 9090})");
    let expression = yeti::parse(tokens);
    let (_, actual) = yeti::evaluate(environment, expression)?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
