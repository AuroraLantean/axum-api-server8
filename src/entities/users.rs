//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12
// but then manually modified by adding Deserialize, Serialize, #[serde(skip_deserializing)], and IntoResponse for Model
use axum::{
  body::Body,
  response::{IntoResponse, Response},
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[serde(skip_deserializing)]
  pub id: i32,
  #[sea_orm(unique)]
  pub name: String,
  pub password: String,
  #[sea_orm(unique)]
  pub email: String,
  pub occupation: Option<String>,
  pub phone: Option<String>,
  pub level: i32,
  #[sea_orm(column_type = "Decimal(Some((26, 9)))")]
  pub balance: Decimal,
  pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoResponse for Model {
  fn into_response(self) -> Response {
    let res = serde_json::to_string_pretty(&self).unwrap();
    Response::new(Body::from(res))
    //into_response()
  }
}
