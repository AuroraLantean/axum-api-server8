use serde::Serialize;
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

pub async fn db() -> Client {
  // see Docker file > Docker Postgres ... to setup a Postgres Docker first
  // $ docker start container_name
  let connection_string =
    "host=localhost port=5431 user=postgres password=password dbname=db_name1";

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

/*Axum sqlx connection pool through shared state
https://stackoverflow.com/questions/77412425/axum-pgconnection-through-shared-state
 */
