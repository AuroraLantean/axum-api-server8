use chrono::DateTime;
use rust_decimal::prelude::*;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::types::{Date, Timestamp};
//-----------== Sea ORM Model
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Default, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32, //u64 is not supported in sqlx
  #[sea_orm(column_type = "Text")]
  #[sea_orm(unique)]
  pub name: String,
  #[sea_orm(column_type = "Text")]
  pub password: String,
  #[sea_orm(column_type = "Text")]
  pub email: String,
  #[sea_orm(column_type = "Text", nullable)]
  pub occupation: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub phone: Option<String>,
  //#[sea_orm(column_type = "Int4", nullable)]
  pub level: i32,
  pub balance: Decimal,
  pub updated_at: DateTimeWithTimeZone, //Nullable<Timestamp>,
}
#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

//-----------== Raw Model
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserRaw {
  pub id: i32, //u64 is not supported in sqlx
  pub name: String,
  pub password: String,
  pub occupation: Option<String>,
  pub email: String,
  pub phone: Option<String>,
  pub priority: Option<i32>,
  #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
  pub balance: Decimal,
  //#[serde(rename="updatedAt")]
  //pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(with = "time::serde::iso8601")]
  pub updated_at: OffsetDateTime,
  //Nullable<Timestamp>,
} // https://time-rs.github.io/book/how-to/parse-dates.html#parsing-into-structs-with-serde
//https://github.com/paupino/rust-decimal?tab=readme-ov-file#serde-with-arbitrary-precision

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct UserList {
  list: Vec<UserRaw>,
}
