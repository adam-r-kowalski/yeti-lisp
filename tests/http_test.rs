use httpdate::fmt_http_date;
use std::time::SystemTime;

use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_default_options() -> Result {
    let env = yeti::core::environment();
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/server {:routes {"/" "Hello Yeti"}})"#).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3000"})"#).await?;
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
        r#"(http/server {:port 3001 :routes {"/" "Hello Yeti"}})"#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3001" :method :get})"#,
    )
    .await?;
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
        (http/server {:port 3002
                      :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3002"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3002/"
          :html [:html [:head] [:body [:ul [:li "first"] [:li "second"]]]]}}
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
        (http/server {:port 3003
                      :routes {"/" (fn [req] [:ul [:li "first"] [:li "second"]])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3003"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3003/"
          :html [:html [:head] [:body [:ul [:li "first"] [:li "second"]]]]}}
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
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3004 :routes {"/" handler}})"#).await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3004"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3004/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3004"}
         :method "GET"
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_query_parameters_in_url() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3005 :routes {"/" handler}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3005?foo=bar&baz=qux"})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3005/?foo=bar&baz=qux"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3005"}
         :method "GET"
         :query {:baz "qux", :foo "bar"}
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_query_parameters_in_map() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3006 :routes {"/" handler}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3006" :query {:foo "bar" :baz "qux"}})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3006/?baz=qux&foo=bar"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3006"}
         :method "GET"
         :query {:baz "qux", :foo "bar"}
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_url_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(http/server {:port 3007 :routes {"/hello/:name" handler}})"#,
    )
    .await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3007/hello/joe"})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3007/hello/joe"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3007"}
         :method "GET"
         :params {:name "joe"}
         :path "/hello/joe"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_form_data() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3009 :routes {"/" handler}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3009" :method :post :form {:foo "bar" :baz "qux"}})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3009/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*"
                   :host "localhost:3009"
                   :content-type "application/x-www-form-urlencoded"
                   :content-length "15"}
         :method "POST"
         :form {:foo "bar"
                :baz "qux"}
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_post_json_data() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3010 :routes {"/" handler}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3010" :method :post :json {:foo "bar" :baz "qux"}})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3010/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*"
                   :host "localhost:3010"
                   :content-type "application/json"
                   :content-length "25"}
         :method "POST"
         :json {:foo "bar"
                :baz "qux"}
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_request_with_headers() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3011 :routes {"/" handler}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3011" :headers {:foo "bar" :baz "qux"}})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3011/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*"
                   :host "localhost:3011"
                   :foo "bar"
                   :baz "qux"}
         :method "GET"
         :path "/"}
        "#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn server_route_returns_json() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/server {:port 3012 :routes {"/" {:foo :bar}}})"#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3012"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "13"
                     :content-type "application/json"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3012/"
          :json {{:foo "bar"}}}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn server_route_function_which_returns_json() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn add [{:json {:lhs lhs :rhs rhs}}]
          {:result (+ lhs rhs)})
        "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3013 :routes {"/" add}})"#).await?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(http/request {:url "http://localhost:3013" :json {:lhs 1 :rhs 2}})"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "application/json"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3013/"
          :json {{:result 3}}}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn server_route_redirect() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (def home {:redirect "/other"})
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (def other [:h1 "Hello World"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (http/server {:port 3014
                      :routes {"/" home
                               "/other" other}})
        "#,
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3014"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "20"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3014/other"
          :html [:html [:head] [:body [:h1 "Hello World"]]]}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn server_route_redirect_with_query_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (def home {:redirect {:url "/other"
                              :query {:name "Joe"}}})
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn other [{:query {:name name}}] [:h1 name])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (http/server {:port 3015
                      :routes {"/" home
                               "/other" other}})
        "#,
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3015"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3015/other?name=Joe"
          :html [:html [:head] [:body [:h1 "Joe"]]]}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_stop_using_handle() -> Result {
    let tokens = yeti::Tokens::from_str("(def s (http/server {:port 3016}))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (environment, _) = yeti::evaluate(environment, expression).await?;
    let tokens = yeti::Tokens::from_str("(http/server-stop s)");
    let expression = yeti::parse(tokens);
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_stop_using_port() -> Result {
    let tokens = yeti::Tokens::from_str("(http/server {:port 3017})");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (environment, _) = yeti::evaluate(environment, expression).await?;
    let tokens = yeti::Tokens::from_str("(http/server-stop {:port 3017})");
    let expression = yeti::parse(tokens);
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_redefine_server() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(http/server {:port 3018 :routes {"/" "Hello Yeti"}})"#,
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3018"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "10"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3018/"
          :text "Hello Yeti"}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(http/server {:port 3018 :routes {"/" "Goodbye Yeti"}})"#,
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/request {:url "http://localhost:3018"})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3018/"
          :text "Goodbye Yeti"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn streaming_response_from_endpoint() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
    (defn handler []
      (let [c (chan)]
        (spawn
          (let [_ (put! c "Hello")
                _ (put! c "Goodbye")
                _ (put! c nil)]
            nil))
        c))
    "#,
    )
    .await?;
    let (env, _) =
        yeti::evaluate_source(env, r#"(http/server {:port 3019 :routes {"/" handler}})"#).await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def response (http/request {:url "http://localhost:3019"}))"#,
    )
    .await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_headers_str = format!(
        r#"
        {{:transfer-encoding "chunked"
          :content-type "text/event-stream"
          :cache-control "no-cache"
          :connection "keep-alive"
          :date "{}"}}
        "#,
        formatted_date
    );
    let (env, expected_headers) = yeti::evaluate_source(env, &expected_headers_str).await?;
    let (env, actual_headers) = yeti::evaluate_source(env, "(:headers response)").await?;
    assert_eq!(actual_headers, expected_headers);
    let (env, actual_status) = yeti::evaluate_source(env, "(:status response)").await?;
    assert_eq!(
        actual_status,
        yeti::Expression::Integer(rug::Integer::from(200))
    );
    let (env, actual_url) = yeti::evaluate_source(env, "(:url response)").await?;
    assert_eq!(
        actual_url,
        yeti::Expression::String("http://localhost:3019/".to_string())
    );
    let (env, _) = yeti::evaluate_source(env, "(def c (:channel response))").await?;
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::String("Hello".to_string()));
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::String("Goodbye".to_string()));
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::Nil);
    let (env, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::Nil);
    let (_, actual) = yeti::evaluate_source(env, "(take! c)").await?;
    assert_eq!(actual, yeti::Expression::Nil);
    Ok(())
}
