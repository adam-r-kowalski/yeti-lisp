use std::sync::Arc;

use forge;
use im::{hashmap, vector, HashMap};
use rug::{Integer, Rational};
use spin::Mutex;

type Result = std::result::Result<(), forge::effect::Effect>;

#[test]
fn evaluate_keyword() -> Result {
    let tokens = forge::Tokens::from_str(":x");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_string() -> Result {
    let tokens = forge::Tokens::from_str(r#""hello""#);
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_integer() -> Result {
    let tokens = forge::Tokens::from_str("5");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_float() -> Result {
    let tokens = forge::Tokens::from_str("3.14");
    let expression = forge::parse(tokens);
    let environment = forge::Environment::new();
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Float(forge::Float::from_str("3.14"));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn evaluate_symbol_bound_to_integer() -> Result {
    let tokens = forge::Tokens::from_str("x");
    let expression = forge::parse(tokens);
    let environment = forge::Environment {
        bindings: hashmap! {
            "x".to_string() => forge::Expression::Integer(Integer::from(5)),
        },
        servers: Arc::new(Mutex::new(HashMap::new())),
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
    let environment = forge::Environment {
        bindings: hashmap! {
            "double".to_string() => forge::Expression::NativeFunction(
              |env, args| {
                let (env, args) = forge::evaluate_expressions(env, args)?;
                match &args[0] {
                  forge::Expression::Integer(i) => Ok((env, forge::Expression::Integer(i * Integer::from(2)))),
                  _ => panic!("Expected integer argument"),
                }
              }
            ),
        },
        servers: Arc::new(Mutex::new(HashMap::new())),
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
    assert_eq!(actual_environment.bindings, expected_environment.bindings);
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
    assert_eq!(actual_environment.bindings, expected_environment.bindings);
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

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let tokens = forge::Tokens::from_str(
        r#"
        (server {:routes {"/" "Hello Forge"}})
        "#,
    );
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
        (server {:port 4000
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
        (server {:port 8080
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
async fn evaluate_shutdown() -> Result {
    let tokens = forge::Tokens::from_str("(server {:port 9090})");
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (environment, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    let tokens = forge::Tokens::from_str("(shutdown {:port 9090})");
    let expression = forge::parse(tokens);
    let (_, actual) = forge::evaluate(environment, expression)?;
    let expected = forge::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_sqlite() -> Result {
    let tokens = forge::Tokens::from_str(r#"(sqlite ":memory:")"#);
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;
    assert!(matches!(actual, forge::Expression::Sqlite(_)));
    Ok(())
}

#[tokio::test]
async fn evaluate_sql_create_table() -> Result {
    let tokens = forge::Tokens::from_str(
        r#"
    (sql {:create-table :fruit
          :with-columns [[:id :int [:not nil]]
                         [:name [:varchar 32] [:not nil]]
                         [:cost :float :null]]})
    "#,
    );
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, actual) = forge::evaluate(environment, expression)?;

    let tokens = forge::Tokens::from_str(
        r#"["CREATE TABLE fruit (id INT NOT NULL, name VARCHAR(32) NOT NULL, cost FLOAT NULL)"]"#,
    );
    let expression = forge::parse(tokens);
    let environment = forge::core::environment();
    let (_, expected) = forge::evaluate(environment, expression)?;

    assert_eq!(actual, expected);
    Ok(())
}
