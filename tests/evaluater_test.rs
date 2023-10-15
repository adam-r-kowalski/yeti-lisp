use forge;
use im::{hashmap, vector, HashMap};
use rug::{Integer, Rational};

type Result = std::result::Result<(), forge::RaisedEffect>;

#[test]
fn evaluate_keyword() -> Result {
    let tokens = forge::Tokens::from_str(":x");
    let expression = forge::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_string() -> Result {
    let tokens = forge::Tokens::from_str(r#""hello""#);
    let expression = forge::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_integer() -> Result {
    let tokens = forge::Tokens::from_str("5");
    let expression = forge::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_float() -> Result {
    let tokens = forge::Tokens::from_str("3.14");
    let expression = forge::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Float(forge::Float::from_str("3.14"));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_symbol_bound_to_integer() -> Result {
    let tokens = forge::Tokens::from_str("x");
    let expression = forge::parse(tokens);
    let environment = hashmap! {
        "x".to_string() => forge::Expression::Integer(Integer::from(5)),
    };
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_symbol_bound_to_function() -> Result {
    let tokens = forge::Tokens::from_str("(double 5)");
    let expression = forge::parse(tokens);
    let environment = hashmap! {
        "double".to_string() => forge::Expression::IntrinsicFunction(
          |env, args| {
            let (env, args) = forge::evaluate_expressions(env, args)?;
            match &args[0] {
              forge::Expression::Integer(i) => Ok((env, forge::Expression::Integer(i * Integer::from(2)))),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
    };
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_add() -> Result {
    let tokens = forge::Tokens::from_str("(+ 5 3)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_if_then_branch() -> Result {
    let tokens = forge::Tokens::from_str("(if true 1 2)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_if_else_branch() -> Result {
    let tokens = forge::Tokens::from_str("(if false 1 2)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_def() -> Result {
    let tokens = forge::Tokens::from_str("(def x 5)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (actual_environment, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert(
        "x".to_string(),
        forge::Expression::Integer(Integer::from(5)),
    );
    assert_eq!(actual_environment, expected_environment);
    Ok(())
}

#[test]
fn evaluate_array() -> Result {
    let tokens = forge::Tokens::from_str("[(+ 1 2) (/ 4 3)]");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Array(vector![
        forge::Expression::Integer(Integer::from(3)),
        forge::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map() -> Result {
    let tokens = forge::Tokens::from_str("{:a (+ 1 2) :b (/ 4 3)}");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(3)),
        forge::Expression::Keyword(":b".to_string()) => forge::Expression::Ratio(Rational::from((Integer::from(4), Integer::from(3)))),
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_key_on_map() -> Result {
    let tokens = forge::Tokens::from_str("(:a {:a 1})");
    let expression = forge::parse(tokens);
    let environment = hashmap! {};
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_map_on_key() -> Result {
    let tokens = forge::Tokens::from_str("({:a 1} :a)");
    let expression = forge::parse(tokens);
    let environment = hashmap! {};
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_assoc() -> Result {
    let tokens = forge::Tokens::from_str("(assoc {} :a 1)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(1)),
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_dissoc() -> Result {
    let tokens = forge::Tokens::from_str("(dissoc {:a 1} :a)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {});
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_merge() -> Result {
    let tokens = forge::Tokens::from_str("(merge {:a 1} {:b 2})");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Map(hashmap! {
        forge::Expression::Keyword(":a".to_string()) => forge::Expression::Integer(Integer::from(1)),
        forge::Expression::Keyword(":b".to_string()) => forge::Expression::Integer(Integer::from(2)),
    });
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_quote() -> Result {
    let tokens = forge::Tokens::from_str("'(1 2)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Call {
        function: Box::new(forge::Expression::Integer(Integer::from(1))),
        arguments: vector![forge::Expression::Integer(Integer::from(2)),],
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_eval() -> Result {
    let tokens = forge::Tokens::from_str("(eval '(+ 1 2))");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Integer(Integer::from(3));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_read_string() -> Result {
    let tokens = forge::Tokens::from_str(r#"(read-string "(+ 1 2)")"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Call {
        function: Box::new(forge::Expression::Symbol("+".to_string())),
        arguments: vector![
            forge::Expression::Integer(Integer::from(1)),
            forge::Expression::Integer(Integer::from(2)),
        ],
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_fn() -> Result {
    let tokens = forge::Tokens::from_str("(fn [x] (* x 2))");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Function {
        parameters: vector![forge::Expression::Symbol("x".to_string()),],
        body: Box::new(forge::Expression::Call {
            function: Box::new(forge::Expression::Symbol("*".to_string())),
            arguments: vector![
                forge::Expression::Symbol("x".to_string()),
                forge::Expression::Integer(Integer::from(2)),
            ],
        }),
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_call_fn() -> Result {
    let tokens = forge::Tokens::from_str("((fn [x] (* x 2)) 5)");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_defn() -> Result {
    let tokens = forge::Tokens::from_str("(defn double [x] (* x 2))");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (actual_environment, actual) = forge::evaluate(environment.clone(), expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let mut expected_environment = environment;
    expected_environment.insert(
        "double".to_string(),
        forge::Expression::Function {
            parameters: vector![forge::Expression::Symbol("x".to_string()),],
            body: Box::new(forge::Expression::Call {
                function: Box::new(forge::Expression::Symbol("*".to_string())),
                arguments: vector![
                    forge::Expression::Symbol("x".to_string()),
                    forge::Expression::Integer(Integer::from(2)),
                ],
            }),
        },
    );
    assert_eq!(actual_environment, expected_environment);
    Ok(())
}

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
