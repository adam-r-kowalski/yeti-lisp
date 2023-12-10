use compiler;

type Result = std::result::Result<(), compiler::effect::Effect>;

#[tokio::test]
async fn evaluate_html_is_module() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "html").await?;
    assert!(matches!(actual, compiler::Expression::Module(_)));
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_only_tag() -> Result {
    let env = compiler::core::environment();
    let (_, actual) = compiler::evaluate_source(env, "(html/to-string [:div])").await?;
    let expected = compiler::Expression::String("<div></div>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_child() -> Result {
    let tokens = compiler::Tokens::from_str(r#"(html/to-string [:ul [:li "hello"]])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::String("<ul><li>hello</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_two_children() -> Result {
    let tokens =
        compiler::Tokens::from_str(r#"(html/to-string [:ul [:li "first"] [:li "second"]])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected =
        compiler::Expression::String("<ul><li>first</li><li>second</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_attribute() -> Result {
    let tokens = compiler::Tokens::from_str(r#"(html/to-string [:div {:class "red"}])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::String(r#"<div class="red"></div>"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_attribute_and_doesnt_need_closing_tag() -> Result {
    let tokens =
        compiler::Tokens::from_str(r#"(html/to-string [:input {:type "checkbox" :name "agree"}])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected =
        compiler::Expression::String(r#"<input name="agree" type="checkbox" />"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_css() -> Result {
    let tokens = compiler::Tokens::from_str(
        r#"
        (html/to-string
         [:style
          {:body {:background-color "red"}}])
        "#,
    );
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected_html = "<style>body { background-color: red; }</style>".to_string();
    let expected = compiler::Expression::String(expected_html);
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_array_of_child() -> Result {
    let tokens =
        compiler::Tokens::from_str(r#"(html/to-string [:ul [[:li "first"] [:li "second"]]])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected =
        compiler::Expression::String("<ul><li>first</li><li>second</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_with_int() -> Result {
    let tokens = compiler::Tokens::from_str(r#"(html/to-string [:ul [:li 1] [:li 2]])"#);
    let expression = compiler::parse(tokens);
    let environment = compiler::core::environment();
    let (_, actual) = compiler::evaluate(environment.clone(), expression).await?;
    let expected = compiler::Expression::String("<ul><li>1</li><li>2</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_from_string_div() -> Result {
    let env = compiler::core::environment();
    let (env, actual) =
        compiler::evaluate_source(env, "(html/from-string \"<div></div>\")").await?;
    let (_, expected) = compiler::evaluate_source(env, "[:html [:head] [:body [:div]]]").await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_from_string_div_with_attribute() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(
        env,
        r#"(html/from-string "<div id=\"foo\" class=\"bar\"></div>")"#,
    )
    .await?;
    let (_, expected) = compiler::evaluate_source(
        env,
        r#"[:html [:head] [:body [:div {:id "foo" :class "bar"}]]]"#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_html_from_string_children() -> Result {
    let env = compiler::core::environment();
    let (env, actual) = compiler::evaluate_source(
        env,
        r#"(html/from-string "<ul><li>first</li><li>second</li></ul>")"#,
    )
    .await?;
    let (_, expected) = compiler::evaluate_source(
        env,
        r#"[:html [:head] [:body [:ul [:li "first"] [:li "second"]]]]"#,
    )
    .await?;
    assert_eq!(actual, expected);
    Ok(())
}
