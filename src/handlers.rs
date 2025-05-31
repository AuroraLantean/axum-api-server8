use axum::{
  Extension, Json,
  body::Body,
  extract::{Path, Query, Request, State},
  http::StatusCode,
  response::{Html, IntoResponse, Redirect, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, to_string_pretty};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fmt::Debug};

use crate::{SharedState, middleware::JwtData, model::User};

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
  "".to_owned() + method.as_str() + "," + &uri.to_string()
}
#[derive(Debug, Deserialize)]
pub struct AddUser {
  pub name: String,
  pub occupation: String,
  pub email: String,
  pub phone: String,
  pub balance: i32,
}
pub async fn add_user(Json(input): Json<AddUser>) -> (StatusCode, Json<User>) {
  let user = User {
    id: 1337, //id: Uuid::new_v4(),
    name: input.name,
    occupation: input.occupation,
    email: input.email,
    phone: input.phone,
    balance: input.balance,
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::CREATED, Json(user)) //Code = `201 Created`
}
pub async fn get_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
    balance: 1000,
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
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
  "intoResponse trait".to_owned() + &id
  //Response::new(Body::new("str".to_owned()))
  //(StatusCode::ACCEPTED, "str")
}
pub async fn fallback_handler() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "404 | Not Found")
}

//#[axum::debug]
pub async fn post_shared_state(
  State(shared_state): State<Arc<Mutex<SharedState>>>,
) -> (StatusCode, Json<SharedState>) {
  let mut state = shared_state.lock().unwrap();
  println!("input state: {:?}", state);
  (*state).token = "xyz".to_owned();
  println!("output state: {:?}", state);
  (StatusCode::OK, Json(state.clone()))
}
pub async fn get_shared_state(
  State(shared_state): State<Arc<Mutex<SharedState>>>,
) -> (StatusCode, Json<SharedState>) {
  let state = shared_state.lock().unwrap();
  println!("shared_state: {:?}", state);
  (StatusCode::OK, Json(state.clone()))
}
pub async fn extension_handler(
  Extension(jwt_data): Extension<Arc<JwtData>>,
) -> (StatusCode, Json<Value>) {
  println!("jwt_data: {:?}", jwt_data);
  (
    StatusCode::OK,
    Json(json!({
      "id": "id",
      "name": "name"
    })),
  )
}
pub async fn redirect_handler() -> impl IntoResponse {
  Redirect::to("/text")
}

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

pub async fn put_user(
  Path(id): Path<String>,
  Json(input): Json<AddUser>,
) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: input.name,
    occupation: String::from("developer"),
    email: input.email,
    phone: input.phone,
    balance: input.balance,
  };
  println!("new user: {:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::OK, Json(user)) //Code = `201 Created`
}
#[derive(Debug, Deserialize)]
pub struct PatchUser {
  pub name: Option<String>,
  pub occupation: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub balance: Option<i32>,
}
pub async fn patch_user(
  Path(id): Path<String>,
  Json(input): Json<PatchUser>,
) -> (StatusCode, Json<User>) {
  let mut user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
    balance: 1000,
  };
  println!("old user: {:?}", user);
  if let Some(name) = input.name {
    user.name = name;
  }
  if let Some(occupation) = input.occupation {
    user.occupation = occupation;
  }
  if let Some(email) = input.email {
    user.email = email;
  }
  if let Some(phone) = input.phone {
    user.phone = phone;
  }
  if let Some(balance) = input.balance {
    user.balance = i32::try_from(balance).expect("msg");
  }
  println!("new user: {:?}", user);
  (StatusCode::OK, Json(user))
}
pub async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
    balance: 1000,
  };
  println!("old user: {:?}", user);

  /*if db.write().unwrap().remove(&id).is_some() {
    StatusCode::NO_CONTENT
  } else {
      StatusCode::NOT_FOUND
  }*/
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
  pub user_id: u32,
  pub team_id: u32,
}
pub async fn customized_path(Path(params): Path<Params>) -> impl IntoResponse {
  axum::Json(params)
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
  axum::Json(payload)
  //Json(dbg!(value));
}

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
