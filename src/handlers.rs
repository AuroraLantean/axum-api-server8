use axum::{
  Json, Router,
  body::Body,
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::{delete, get, patch, post, put},
};
use serde::{Deserialize, Serialize};

pub async fn root() -> &'static str {
  "Hello Root!"
}
pub async fn post_handler() -> Response {
  Response::builder()
    .status(StatusCode::CREATED)
    .header("Content-Type", "application/json")
    .body(Body::from(r#"{"name":"john"}"#))
    .unwrap()
  //(StatusCode::CREATED, "New Post Added!")
} //[("Content-Type":"application/json")], r#"{"name":"john"}"#,

pub async fn add_user(Json(payload): Json<AddUser>) -> (StatusCode, Json<User>) {
  let user = User {
    id: 1337, //id: Uuid::new_v4(),
    username: payload.username,
    balance: payload.balance,
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::CREATED, Json(user)) //Code = `201 Created`
}
pub async fn read_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<u64>().unwrap(),
    username: String::from("username"),
    balance: 1000,
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}
// the input to our `add_user` handler
#[derive(Debug, Deserialize)]
pub struct AddUser {
  username: String,
  balance: u64,
}
#[derive(Debug, Serialize, Clone)]
pub struct User {
  id: u64, // Uuid,
  username: String,
  balance: u64,
}
