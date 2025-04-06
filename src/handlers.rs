use axum::{
  Json, Router,
  body::Body,
  extract::{Path, Query},
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
    username: String::from("JohnDoe"),
    balance: 1000,
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
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

pub async fn update_user(
  Path(id): Path<String>,
  Json(input): Json<UpdateUser>,
) -> (StatusCode, Json<User>) {
  let mut user = User {
    id: id.parse::<u64>().unwrap(),
    username: String::from("JohnDoe"),
    balance: 1000,
  };
  println!("old user: {:?}", user);

  if let Some(text) = input.username {
    user.username = text;
  }
  if let Some(balance) = input.balance {
    user.balance = balance;
  }
  println!("new user: {:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}

pub async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<u64>().unwrap(),
    username: String::from("JohnDoe"),
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

// the input to our `add_user` handler
#[derive(Debug, Deserialize)]
pub struct AddUser {
  username: String,
  balance: u64,
}
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
  username: Option<String>,
  balance: Option<u64>,
}
#[derive(Debug, Serialize, Clone)]
pub struct User {
  pub id: u64, // Uuid,
  pub username: String,
  pub balance: u64,
}
