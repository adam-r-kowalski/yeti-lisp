/*
use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn sql_connect() -> Result {
    let env = yeti::core::environment();
    let (_, actual) = yeti::evaluate_source(env, r#"(sql/connect)"#)?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    Ok(())
}

#[tokio::test]
async fn create_table_string() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
    (sql/string
     {:create-table :fruit
      :with-columns [[:id :int [:not nil]]
                     [:name [:varchar 32] [:not nil]]
                     [:cost :float :null]]})
    "#,
    )?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"["CREATE TABLE fruit (id INT NOT NULL, name VARCHAR(32) NOT NULL, cost FLOAT NULL)"]"#,
    )?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn get_all_table_names() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, r#"(def conn (sql/connect))"#)?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
    (sql/execute! conn
     {:create-table :fruit
      :with-columns [[:id :int [:not nil]]
                     [:name [:varchar 32] [:not nil]]
                     [:cost :float :null]]})
    "#,
    )?;
    let (env, actual) = yeti::evaluate_source(env, "(sql/tables conn)")?;
    let (_, expected) = yeti::evaluate_source(env, r#"[{:name "fruit"}]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn insert_into_vector_syntax_string() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
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
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"["INSERT INTO properties (name, surname, age) VALUES (?, ?, ?), (?, ?, ?), (?, ?, ?)"
            "Jon" "Smith" 34 "Andrew" "Cooper" 12 "Jane" "Daniels" 56]"#,
    )?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn select_string() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
    (sql/string
     {:select [:a :b :c]
      :from :foo
      :where [:= :foo.a "baz"]})
    "#,
    )?;
    let (_, expected) =
        yeti::evaluate_source(env, r#"["SELECT a, b, c FROM foo WHERE foo.a = ?" "baz"]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn select_single_column() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
    (sql/string
     {:select :a
      :from :foo
      :where [:= :a "baz"]})
    "#,
    )?;
    let (_, expected) = yeti::evaluate_source(env, r#"["SELECT a FROM foo WHERE a = ?" "baz"]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn select_not_equal() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
    (sql/string
     {:select :a
      :from :foo
      :where [:!= :a "baz"]})
    "#,
    )?;
    let (_, expected) = yeti::evaluate_source(env, r#"["SELECT a FROM foo WHERE a != ?" "baz"]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn insert_and_select() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, r#"(def conn (sql/connect))"#)?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
    (sql/execute! conn
     {:create-table :properties
      :with-columns [[:name [:varchar 32] [:not nil]]
                     [:surname [:varchar 32] [:not nil]]
                     [:age :int [:not nil]]]})
    "#,
    )?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
    (sql/execute! conn
     {:insert-into :properties
      :columns [:name :surname :age]
      :values [["Jon" "Smith" 34]
               ["Andrew" "Cooper" 12]
               ["Jane" "Daniels" 56]]})
    "#,
    )?;
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
    (sql/query conn
     {:select [:name, :surname]
      :from :properties
      :where [:= :age 34]})
    "#,
    )?;
    let (_, expected) = yeti::evaluate_source(env, r#"[{:name "Jon" :surname "Smith"}]"#)?;
    assert_eq!(actual, expected);
    Ok(())
}
*/
