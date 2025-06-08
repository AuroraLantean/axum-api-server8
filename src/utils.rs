use axum::{
  Json,
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use sea_orm::DbErr;

use crate::entities::users;

pub struct PaginationOut(pub Vec<users::Model>, pub u64);
impl IntoResponse for PaginationOut {
  fn into_response(self) -> Response {
    let json_string = serde_json::to_string_pretty(&self.0).unwrap();
    let out = format!("vec: {}, num_pages: {}", json_string, self.1);
    Response::new(Body::from(out))
    //into_response()
  }
}
//See Axum examples/anyhow-error
pub struct DbErrOut(DbErr);
impl IntoResponse for DbErrOut {
  fn into_response(self) -> Response {
    let res = self.0.to_string();
    Response::new(Body::from(res))
  }
}
impl<E> From<E> for DbErrOut
where
  E: Into<DbErr>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

pub fn db_err(str: &str) -> DbErrOut {
  DbErrOut(DbErr::Custom(str.to_owned()))
}

// Utility function for mapping any error into a `500 Internal Server Error`
fn _internal_error<E>(err: E) -> (StatusCode, String)
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
