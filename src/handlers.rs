use crate::{SharedState, SharedUser, middleware::MiddlewareData};
use axum::{
  Extension, Json,
  body::Body,
  extract::{Form, Path, Query, Request, State},
  http::{StatusCode, Uri},
  response::{Html, IntoResponse, Redirect, Response},
};
use axum_session::Session;
use axum_session_sqlx::SessionSqlitePool;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, to_string_pretty};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fmt::Debug};

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
struct Output {
  id: String,
  name: String,
  balance: i32,
}
impl IntoResponse for Output {
  fn into_response(self) -> Response {
    let res = serde_json::to_string_pretty(&self).unwrap();
    Response::new(Body::from(res))
  }
}
//Return dynamic custom type
pub async fn into_response_trait_custom_output(Path(id): Path<String>) -> impl IntoResponse {
  Output {
    id,
    name: "John Doe".to_owned(),
    balance: 300,
  } // "out: ".to_owned() + &id + &name + balance
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

//------------== Middleware
pub async fn middleware_data_handler(
  Extension(middleware_data): Extension<Arc<MiddlewareData>>,
) -> impl IntoResponse {
  println!("{:?}", middleware_data);
  (StatusCode::OK, Json(middleware_data))
}
pub async fn get_state_in_middleware_handler() -> impl IntoResponse {
  println!("get_state_in_middleware_handler");
  (StatusCode::OK, "get_state_in_middleware_handler")
}
pub async fn get_state_handler(
  State((shared_state, shared_user)): State<(SharedState, SharedUser)>,
) -> impl IntoResponse {
  println!("get_state_handler");
  let txt = format!(
    "id: {}, mesg: {}, name: {}, age: {}",
    shared_state.num, shared_state.mesg, shared_user.name, shared_user.age
  );
  (StatusCode::OK, txt)
}
pub async fn get_mut_shared_state_handler(
  State(shared_state_mut): State<Arc<Mutex<SharedState>>>,
) -> impl IntoResponse {
  let state = shared_state_mut.lock().unwrap();
  println!("shared_state_mut: {:?}", state);
  (StatusCode::OK, Json(state.clone())) //Json(shared_state)
}
#[derive(Debug, Deserialize)]
pub struct NewState {
  mesg: String,
  num: u32,
}
pub async fn post_mut_shared_state_handler(
  State(shared_state): State<Arc<Mutex<SharedState>>>,
  Json(input): Json<NewState>,
) -> impl IntoResponse {
  let mut state = shared_state.lock().unwrap();
  println!("state: {:?}", state);

  println!("original json: {:?}", input);
  (*state).mesg = input.mesg;
  (*state).num = input.num;
  println!("new state: {:?}", state);
  (StatusCode::OK, Json(state.clone()))
}

pub async fn redirect_handler() -> impl IntoResponse {
  Redirect::to("/text")
}
pub async fn fallback_handler() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "404 | Not Found")
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
  name: String,
  email: String,
  phone: u32,
} //Deserialize for input
pub async fn contact_form_handler(
  Form(contact_form): Form<ContactForm>,
) -> (StatusCode, Json<ContactForm>) {
  println!("contact_form: {:?}", contact_form);
  (StatusCode::OK, Json(contact_form))
}

//----------------==
#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
  user_id: u32,
  team_id: u32,
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

//----------------== Error ;;;
#[derive(Debug, Serialize, Clone)]
pub struct Error {
  code: u64, // Uuid,
  mesg: String,
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

//---------------== Session Handler
async fn _session_set_handler(session: Session<SessionSqlitePool>) -> impl IntoResponse {
  session.set("u_id", 1);

  let s_id = session.get_session_id();
  println!("s_id: {:?}", s_id);

  (StatusCode::OK, s_id.to_string())
}
async fn _session_get_handler(session: Session<SessionSqlitePool>) -> impl IntoResponse {
  let value: String = session.get("u_id").unwrap();
  (StatusCode::OK, value)
}
