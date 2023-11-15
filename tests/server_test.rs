use std::collections::HashMap;

use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(server/start {:routes {"/" "Hello Yeti"}})"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:3000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "Hello Yeti");
    Ok(())
}

#[tokio::test]
async fn http_get() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(server/start {:port 3001 :routes {"/" "Hello Yeti"}})"#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (_, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3001")"#).await?;
    assert_eq!(actual, yeti::Expression::String("Hello Yeti".to_string()));
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_string_route() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 4000
                       :routes {"/" "Hello Yeti"}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:4000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "Hello Yeti");
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_html_route() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 3030
                       :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:3030")
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
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
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
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:10080")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10080"}, :method "GET", :path "/"}</p>"#
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
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:10090?foo=bar&baz=qux")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10090"}, :method "GET", :path "/", :query {:baz "qux", :foo "bar"}}</p>"#
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
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let body = reqwest::get("http://localhost:10070/hello/joe")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :host "localhost:10070"}, :method "GET", :params {:name "joe"}, :path "/hello/joe"}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_server_form_data() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 10040
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let client = reqwest::Client::new();
    let body = client
        .post("http://localhost:10040")
        .form(&[("foo", "bar"), ("baz", "qux")])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:form {:baz "qux", :foo "bar"}, :headers {:accept "*/*", :content-length "15", :content-type "application/x-www-form-urlencoded", :host "localhost:10040"}, :method "POST", :path "/"}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_server_post_json_data() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (server/start {:port 10030
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    map.insert("baz", "qux");
    let body = client
        .post("http://localhost:10030")
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(
        body,
        r#"<p>{:headers {:accept "*/*", :content-length "25", :content-type "application/json", :host "localhost:10030"}, :json {:baz "qux", :foo "bar"}, :method "POST", :path "/"}</p>"#
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_stop() -> Result {
    let tokens = yeti::Tokens::from_str("(def s (server/start {:port 9090}))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (environment, _) = yeti::evaluate(environment, expression).await?;
    let tokens = yeti::Tokens::from_str("(server/stop s)");
    let expression = yeti::parse(tokens);
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
