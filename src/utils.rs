use axum::{
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use sea_orm::DbErr;

pub struct DbErrOut(DbErr);
impl IntoResponse for DbErrOut {
  fn into_response(self) -> Response {
    let res = self.0.to_string();
    //let res = serde_json::to_string_pretty(&self.0).unwrap();
    Response::new(Body::from(res))
    //into_response()
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
