use axum_session::{Key, SessionConfig, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use serde::Serialize;
use sqlx::{Executor, Pool, Sqlite};
use tokio_postgres::{Client, NoTls};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DbClient {
  // Fields related to the database connection
  pub client: Client,
}
#[derive(Clone, Debug, Serialize)]
pub struct DbPool {
  // Fields related to the database connection pool
}
#[derive(Clone, Debug, Serialize)]
pub struct Config {
  // Fields related to configuration information
}

pub async fn tokio_postgres1() -> Client {
  // see Readme or Docker file > Docker Postgres ... to setup a Postgres Docker first
  // $ docker start container_name
  let connection_string = dotenvy::var("DB_POSTGRES_DOCKER_STRING")
    .expect("DB_POSTGRES_DOCKER_STRING not found in env file");
  println!("connection_string: {}", connection_string);
  //let connection_string = "host=localhost port=5431 user=postgres password=password dbname=db_name1";

  let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
    .await
    .expect("database connection failed");
  println!("database connection successful");

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("DB error: {}", e)
    }
  });
  client
}

//---------------== Sqlite
/*Axum sqlx connection pool through shared state
https://stackoverflow.com/questions/77412425/axum-pgconnection-through-shared-state
 */
pub async fn _sqlx_sqlite1() -> Pool<Sqlite> {
  let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
    .await
    .unwrap();

  pool
    .execute(
      "CREATE TABLE IF NOT EXISTS user(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT,
  password TEXT
  )",
    )
    .await
    .unwrap();
  pool
}

// The server makes a session for the user and stores session data on the server-side. The client holds only a session identifier, typically in a cookie. Performance Issues at Scale: The dependency on database interactions for every session validation can introduce latency
pub async fn _session(pool: Pool<Sqlite>) -> SessionStore<SessionSqlitePool> {
  let config = SessionConfig::default()
    .with_table_name("session_table")
    .with_key(Key::generate()); //key to set session cookie encryption key

  let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config)
    .await
    .unwrap();
  session_store
}
