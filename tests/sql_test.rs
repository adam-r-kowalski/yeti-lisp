use forge;

type Result = std::result::Result<(), forge::effect::Effect>;

#[tokio::test]
async fn evaluate_sqlite() -> Result {
    let env = forge::core::environment();
    let (_, actual) = forge::evaluate_source(env, r#"(sqlite ":memory:")"#)?;
    assert!(matches!(actual, forge::Expression::Sqlite(_)));
    Ok(())
}

#[tokio::test]
async fn evaluate_sql_create_table() -> Result {
    let env = forge::core::environment();
    let (env, actual) = forge::evaluate_source(
        env,
        r#"
    (sql {:create-table :fruit
          :with-columns [[:id :int [:not nil]]
                         [:name [:varchar 32] [:not nil]]
                         [:cost :float :null]]})
    "#,
    )?;
    let (_, expected) = forge::evaluate_source(
        env,
        r#"["CREATE TABLE fruit (id INT NOT NULL, name VARCHAR(32) NOT NULL, cost FLOAT NULL)"]"#,
    )?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_query_create_table() -> Result {
    let env = forge::core::environment();
    let (env, _) = forge::evaluate_source(env, r#"(def db (sqlite ":memory:"))"#)?;
    let (env, _) = forge::evaluate_source(
        env,
        r#"
    (query db {:create-table :fruit
               :with-columns [[:id :int [:not nil]]
                              [:name [:varchar 32] [:not nil]]
                              [:cost :float :null]]})
    "#,
    )?;
    let (env, actual) = forge::evaluate_source(env, "(tables db)")?;
    let (_, expected) = forge::evaluate_source(env, r#"["fruit"]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}
