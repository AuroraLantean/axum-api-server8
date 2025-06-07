use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

//-----------== Sea ORM Model: NOT TO USE! Use SeaORM CLI to generate model instead!
//https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/#column-type
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Default, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32, //u64 is not supported in sqlx
  #[sea_orm(unique)]
  pub name: String,
  pub password: String,
  pub email: String,
  //#[sea_orm(column_type = "Text")]
  //#[sea_orm(column_type = "Text", nullable)]
  pub occupation: Option<String>,
  pub phone: Option<String>,
  pub level: i32,
  pub balance: Decimal,
  pub updated_at: DateTimeWithTimeZone,
}
#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

//-----------== Raw Model: Should be copied from generated Entity file then add Serialize, Deserialize
//use time::OffsetDateTime;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserB {
  pub id: i32, //u64 is not supported in sqlx
  pub name: String,
  pub password: String,
  pub email: String,
  pub occupation: Option<String>,
  pub phone: Option<String>,
  pub level: i32,
  pub balance: Decimal,
  //#[serde(rename="updatedAt")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  //#[serde(with = "time::serde::iso8601")]
  //pub updated_at: OffsetDateTime,
  //Nullable<Timestamp>,
} // https://time-rs.github.io/book/how-to/parse-dates.html#parsing-into-structs-with-serde
//https://github.com/paupino/rust-decimal?tab=readme-ov-file#serde-with-arbitrary-precision
