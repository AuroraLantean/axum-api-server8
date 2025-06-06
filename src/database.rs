use axum_session::{Key, SessionConfig, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use sqlx::{Executor, Pool, Postgres, Sqlite, postgres::PgPoolOptions};
use std::time::Duration;

use bb8::Pool as PoolBB8;
use bb8_postgres::PostgresConnectionManager;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio_postgres::NoTls;

fn get_db_connn_str(str: &str) -> String {
  let connection_str = dotenvy::var(str).expect(str);
  println!("postgres conn_str: {}", connection_str);
  connection_str
}
//---------------== Postgres
// see Readme or Docker file > Docker Postgres ... to setup a Postgres Docker first
// $ docker start container_name

pub type _BB8Pool = PoolBB8<PostgresConnectionManager<NoTls>>;
pub async fn _tokio_postgres1() -> _BB8Pool {
  let conn_str = get_db_connn_str("DB_POSTGRES_DOCKER_TOKIO");
  //Axum repo/examples/tokio-postgres
  let manager = PostgresConnectionManager::new_from_stringlike(conn_str, NoTls).unwrap();
  let pool_bb8 = PoolBB8::builder().build(manager).await.unwrap();
  pool_bb8
  /*let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
    .await
    .expect("database connection failed");
  println!("database connection successful via tokio_postgres1");

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("DB error: {}", e)
    }
  });
  client*/
}

//https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/
//Each time you call execute or query_one/all on it, a connection will be acquired and released from the pool.
pub type SeaPool = DatabaseConnection;
pub async fn sea_orm_db() -> SeaPool {
  let conn_str = get_db_connn_str("DB_POSTGRES_DOCKER_SQLX");
  //db_uri should be protocol://username:password@host/database
  let mut opt = ConnectOptions::new(conn_str);
  opt
    .max_connections(10)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(false)
    .sqlx_logging_level(log::LevelFilter::Info);
  //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

  let db_conn = Database::connect(opt).await.expect("sea_orm connect");
  db_conn
}
pub async fn _sqlx_postgres1() -> Pool<Postgres> {
  let conn_str = get_db_connn_str("DB_POSTGRES_DOCKER_SQLX");

  let db_pool = PgPoolOptions::new()
    .max_connections(16)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&conn_str)
    .await
    .expect("could not connect to database");
  println!("database connection successful via sqlx_postgres1");
  db_pool
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
