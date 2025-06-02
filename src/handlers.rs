use axum::{
  Extension, Json,
  body::Body,
  extract::{Form, Path, Query, Request},
  http::{StatusCode, Uri},
  response::{Html, IntoResponse, Redirect, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, to_string_pretty};
use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug};

use crate::middleware::JwtData;

pub async fn root() -> &'static str {
  "Root!"
}
pub async fn html() -> Html<&'static str> {
  Html("<h1>Hello, World!</h1>")
}

pub async fn query_params(Query(params): Query<HashMap<String, String>>) -> &'static str {
  for k in params.keys() {
    println!("key is {}", k)
  }
  for v in params.values() {
    println!("value is {}", v)
  }
  "query_params!"
}
//good for all methods
pub async fn request_params(req: Request) -> String {
  let headers = req.headers();
  let method = req.method();
  let uri = req.uri();
  println!("headers: {:?}", headers);
  println!("method: {:?}", method);
  println!("uri: {:?}", uri);
  "".to_owned() + method.as_str() + " " + &uri.to_string()
}

pub async fn dynamic_json_output(Path(id): Path<String>) -> Json<Value> {
  //serde_json::{Value, json};
  Json(json!({
      "id": id,
      "name": "john",
      "balance": 1000,
  }))
}

#[derive(Serialize)]
#[allow(dead_code)]
struct Output {
  id: String,
  name: String,
  balance: i32,
}
pub async fn resp_output(Path(id): Path<String>) -> Response {
  let output = Output {
    id,
    name: "John Doe".to_owned(),
    balance: 34,
  };
  let json_data = to_string_pretty(&output).unwrap();
  Response::new(Body::new(json_data))
  //Response::new(Body::new("str".to_owned()))
  //Response::new(Body::from("Hello World!"))
}
//Dynamic output
pub async fn into_response_trait_dynamic_output(Path(id): Path<String>) -> impl IntoResponse {
  "intoResponse trait: ".to_owned() + &id
  //Response::new(Body::new("str".to_owned()))
  //(StatusCode::ACCEPTED, "str")
}
pub async fn fallback_handler() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "404 | Not Found")
}

pub async fn extension_handler(
  Extension(jwt_data): Extension<Arc<JwtData>>,
) -> (StatusCode, Json<Arc<JwtData>>) {
  println!("jwt_data: {:?}", jwt_data);
  (StatusCode::OK, Json(jwt_data))
}
pub async fn redirect_handler() -> impl IntoResponse {
  Redirect::to("/text")
}
pub async fn user_profile() -> impl IntoResponse {
  (StatusCode::OK, "user_profile") //Json(state)
}
pub async fn user_setting() -> impl IntoResponse {
  (StatusCode::OK, "user_setting")
}

pub async fn about_handler() -> impl IntoResponse {
  (StatusCode::OK, "about")
}
pub async fn wildcard_handler(Path(path): Path<String>) -> impl IntoResponse {
  println!("path: {:?}", path);
  (StatusCode::OK, "wildcard: ".to_owned() + &path)
}
pub async fn uri_handler(uri: Uri) -> impl IntoResponse {
  println!("uri: {:?}", uri);
  (StatusCode::OK, "uri: ".to_owned() + uri.path())
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactForm {
  pub name: String,
  pub email: String,
  pub phone: u32,
} //Deserialize for input
pub async fn contact_form_handler(
  Form(contact_form): Form<ContactForm>,
) -> (StatusCode, Json<ContactForm>) {
  println!("contact_form: {:?}", contact_form);
  (StatusCode::OK, Json(contact_form))
}

//----------------==
#[allow(dead_code)]
pub async fn list_users() -> (StatusCode, Json<Value>) {
  (StatusCode::FOUND, Json(Value::Null))
}
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
  //#[serde(default, deserialize_with = "empty_string_as_none")]
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}
//{{host}}/users?offset=1&limit=100
pub async fn query_users(pagination: Query<Pagination> /*, State(db): State<Db> */) {
  //let todos = db.read().unwrap();
  println!("pagination.offset: {:?}", pagination.offset.unwrap_or(0));
  println!(
    "pagination.limit: {:?}",
    pagination.limit.unwrap_or(usize::MAX)
  );
  /*let todos = todos
      .values()
      .skip(pagination.offset.unwrap_or(0))
      .take(pagination.limit.unwrap_or(usize::MAX))
      .cloned()
      .collect::<Vec<_>>();

  Json(todos)*/
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
  pub user_id: u32,
  pub team_id: u32,
}
pub async fn customized_path(Path(params): Path<Params>) -> impl IntoResponse {
  Json(params)
}
pub async fn post_raw1() -> Response {
  Response::builder()
    .status(StatusCode::CREATED)
    .header("Content-Type", "application/json")
    .body(Body::from(r#"{"name":"john"}"#))
    .expect("response builder")
  //(StatusCode::CREATED, "New Post Added!")
} //[("Content-Type":"application/json")], r#"{"name":"john"}"#,

pub async fn custom_extractor(Json(value): Json<Value>) -> (StatusCode, Json<Error>) {
  println!("value: {:?}", value);
  let err = Error {
    code: 10013,
    mesg: "my_err_message".to_owned(),
  };
  println!("err: {:?}", err);
  (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
}
pub async fn custom_extractor2(Json(value): Json<Value>) -> impl IntoResponse {
  println!("value: {:?}", value);
  let payload = json!({
      "message": "m", //rejection.body_text(),
      "origin": "custom_extractor",
      "path": "path",
  });
  println!("err: {:?}", payload);
  Json(payload)
  //Json(dbg!(value));
}

//----------------==
#[derive(Debug, Serialize, Clone)]
pub struct Error {
  pub code: u64, // Uuid,
  pub mesg: String,
}

pub async fn internal_error() -> Result<(), AppError> {
  try_thing()?;
  Ok(())
}
fn try_thing() -> Result<(), anyhow::Error> {
  anyhow::bail!("it failed!")
}
// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("internal error: {}", self.0),
    )
      .into_response()
  }
}
// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
