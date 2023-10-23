use forge;

type Result = std::result::Result<(), forge::effect::Effect>;

#[test]
fn evaluate_html_with_only_tag() -> Result {
    let tokens = forge::Tokens::from_str("(html [:div])");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::String("<div></div>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_child() -> Result {
    let tokens = forge::Tokens::from_str(r#"(html [:ul [:li "hello"]])"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::String("<ul><li>hello</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_two_children() -> Result {
    let tokens = forge::Tokens::from_str(r#"(html [:ul [:li "first"] [:li "second"]])"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::String("<ul><li>first</li><li>second</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_attribute() -> Result {
    let tokens = forge::Tokens::from_str(r#"(html [:div {:class "red"}])"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::String(r#"<div class="red"></div>"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_attribute_and_doesnt_need_closing_tag() -> Result {
    let tokens = forge::Tokens::from_str(r#"(html [:input {:type "checkbox" :name "agree"}])"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected =
        forge::Expression::String(r#"<input name="agree" type="checkbox" />"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_css() -> Result {
    let tokens = forge::Tokens::from_str(
        r#"
        (html
         [:style
          {:body {:background-color "red"}}])
        "#,
    );
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected_html = "<style>body { background-color: red; }</style>".to_string();
    let expected = forge::Expression::String(expected_html);
    assert_eq!(actual, expected);
    Ok(())
}
