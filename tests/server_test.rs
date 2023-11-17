use httpdate::fmt_http_date;
use std::collections::HashMap;
use std::time::SystemTime;

use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let env = yeti::core::environment();
    let (env, actual) =
        yeti::evaluate_source(env, r#"(server/start {:routes {"/" "Hello Yeti"}})"#).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3000")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "10"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3000/"
          :text "Hello Yeti"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_string_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3001
                       :routes {"/" "Hello Yeti"}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3001")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "10"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3001/"
          :text "Hello Yeti"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_html_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3002
                       :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3002")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3002/"
          :text "<ul><li>first</li><li>second</li></ul>"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_function_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3003
                       :routes {"/" (fn [req] [:ul [:li "first"] [:li "second"]])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3003")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3003/"
          :text "<ul><li>first</li><li>second</li></ul>"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_show_request_as_str() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3004
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3004")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "83"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3004/"
          :text "<p>{{:headers {{:accept \"*/*\", :host \"localhost:3004\"}}, :method \"GET\", :path \"/\"}}</p>"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_query_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3005
                       :routes {"/" (fn [req] [:p (str req)])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/get "http://localhost:3005/?foo=bar&baz=qux")"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "116"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3005/?foo=bar&baz=qux"
          :text "<p>{{:headers {{:accept \"*/*\", :host \"localhost:3005\"}}, :method \"GET\", :path \"/\", :query {{:baz \"qux\", :foo \"bar\"}}}}</p>"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_url_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3006
                       :routes {"/hello/:name" (fn [req] [:p (str req)])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/get "http://localhost:3006/hello/joe")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "115"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3006/hello/joe"
          :text "<p>{{:headers {{:accept \"*/*\", :host \"localhost:3006\"}}, :method \"GET\", :params {{:name \"joe\"}}, :path \"/hello/joe\"}}</p>"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
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
