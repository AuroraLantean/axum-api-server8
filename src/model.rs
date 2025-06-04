use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::types::{Date, Timestamp};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
  pub id: i32, //u64 is not supported in sqlx
  pub name: String,
  pub password: String,
  pub occupation: String,
  pub email: String,
  pub phone: String,
  pub priority: i32,
  #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
  pub balance: Decimal,
  #[serde(with = "time::serde::iso8601")]
  pub updated_at: OffsetDateTime,
} // https://time-rs.github.io/book/how-to/parse-dates.html#parsing-into-structs-with-serde
//https://github.com/paupino/rust-decimal?tab=readme-ov-file#serde-with-arbitrary-precision

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct UserList {
  list: Vec<User>,
}
