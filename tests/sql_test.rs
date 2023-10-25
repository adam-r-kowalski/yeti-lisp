use forge;

type Result = std::result::Result<(), forge::effect::Effect>;

#[tokio::test]
async fn sql_connect() -> Result {
    let env = forge::core::environment();
    let (_, actual) = forge::evaluate_source(env, r#"(sql/connect)"#)?;
    assert!(matches!(actual, forge::Expression::Sqlite(_)));
    Ok(())
}

#[tokio::test]
async fn create_table_string() -> Result {
    let env = forge::core::environment();
    let (env, actual) = forge::evaluate_source(
        env,
        r#"
    (sql/string
     {:create-table :fruit
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
async fn get_all_table_names() -> Result {
    let env = forge::core::environment();
    let (env, _) = forge::evaluate_source(env, r#"(def conn (sql/connect))"#)?;
    let (env, _) = forge::evaluate_source(
        env,
        r#"
    (sql/execute! conn
     {:create-table :fruit
      :with-columns [[:id :int [:not nil]]
                     [:name [:varchar 32] [:not nil]]
                     [:cost :float :null]]})
    "#,
    )?;
    let (env, actual) = forge::evaluate_source(env, "(sql/tables conn)")?;
    let (_, expected) = forge::evaluate_source(env, r#"[{:name "fruit"}]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn insert_into_vector_syntax_string() -> Result {
    let env = forge::core::environment();
    let (env, actual) = forge::evaluate_source(
        env,
        r#"
    (sql/string
     {:insert-into :properties
      :columns [:name :surname :age]
      :values [["Jon" "Smith" 34]
               ["Andrew" "Cooper" 12]
               ["Jane" "Daniels" 56]]})
    "#,
    )?;
    let (_, expected) = forge::evaluate_source(
        env,
        r#"["INSERT INTO properties (name, surname, age) VALUES (?, ?, ?), (?, ?, ?), (?, ?, ?)"
            "Jon" "Smith" 34 "Andrew" "Cooper" 12 "Jane" "Daniels" 56]"#,
    )?;
    assert_eq!(actual, expected);
    Ok(())
}
