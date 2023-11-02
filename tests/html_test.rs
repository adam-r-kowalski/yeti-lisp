use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[test]
fn evaluate_html_with_only_tag() -> Result {
    let tokens = yeti::Tokens::from_str("(html/string [:div])");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::String("<div></div>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_child() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(html/string [:ul [:li "hello"]])"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::String("<ul><li>hello</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_two_children() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(html/string [:ul [:li "first"] [:li "second"]])"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::String("<ul><li>first</li><li>second</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_attribute() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(html/string [:div {:class "red"}])"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::String(r#"<div class="red"></div>"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_attribute_and_doesnt_need_closing_tag() -> Result {
    let tokens =
        yeti::Tokens::from_str(r#"(html/string [:input {:type "checkbox" :name "agree"}])"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected =
        yeti::Expression::String(r#"<input name="agree" type="checkbox" />"#.to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_css() -> Result {
    let tokens = yeti::Tokens::from_str(
        r#"
        (html/string
         [:style
          {:body {:background-color "red"}}])
        "#,
    );
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected_html = "<style>body { background-color: red; }</style>".to_string();
    let expected = yeti::Expression::String(expected_html);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_html_with_array_of_child() -> Result {
    let tokens = yeti::Tokens::from_str(r#"(html/string [:ul [[:li "first"] [:li "second"]]])"#);
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (_, actual) = yeti::evaluate(environment.clone(), expression)?;
    let expected = yeti::Expression::String("<ul><li>first</li><li>second</li></ul>".to_string());
    assert_eq!(actual, expected);
    Ok(())
}
