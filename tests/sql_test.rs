use forge;

type Result = std::result::Result<(), forge::effect::Effect>;

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
