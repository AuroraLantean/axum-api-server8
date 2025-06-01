use axum::{
  Json,
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use bcrypt;
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::model::User;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AddUser {
  pub name: String,
  pub password: String,
  pub occupation: String,
  pub email: String,
  pub phone: String,
} //in postman: Body > raw: {...}
pub async fn add_user(
  State(client): State<Arc<Client>>,
  Json(input): Json<AddUser>,
) -> impl IntoResponse {
  println!("add_user: {:?}", input);
  let hashed_pw = bcrypt::hash(input.password, 10).unwrap();
  println!("hashed_pw: {:?}", hashed_pw);

  client
    .execute(
      "INSERT INTO users (name, password) VALUES ($1, $2)",
      &[&input.name, &hashed_pw],
    )
    .await
    .unwrap();

  /*let user = User {
    id: 1337, //id: Uuid::new_v4(),
    name: input.name,
    password: hashed_pw,
    occupation: input.occupation,
    email: input.email,
    phone: input.phone,
    balance: input.balance,
  };
  println!("{:?}", user);
  db.write().unwrap().insert(user.id, user.clone());*/
  (StatusCode::CREATED, "Success".to_owned()) //Code = `201 Created`
}
//docker exec -it postgres1 psql -U postgres
//\c db_name1
//SELECT * FROM db_name1;

pub async fn get_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    password: String::from("password"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
  };
  println!("{:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}

pub async fn put_user(
  Path(id): Path<String>,
  Json(input): Json<AddUser>,
) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: input.name,
    password: input.password,
    occupation: String::from("developer"),
    email: input.email,
    phone: input.phone,
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
}
pub async fn patch_user(
  Path(id): Path<String>,
  Json(input): Json<PatchUser>,
) -> (StatusCode, Json<User>) {
  let mut user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    password: String::from("password"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
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
  /*if let Some(balance) = input.balance {
    user.balance = i32::try_from(balance).expect("msg");
  }*/
  println!("new user: {:?}", user);
  (StatusCode::OK, Json(user))
}
pub async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: String::from("JohnDoe"),
    password: String::from("password"),
    occupation: String::from("developer"),
    email: String::from("john@crypto.com"),
    phone: String::from("1234"),
  };
  println!("old user: {:?}", user);

  /*if db.write().unwrap().remove(&id).is_some() {
    StatusCode::NO_CONTENT
  } else {
      StatusCode::NOT_FOUND
  }*/
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}
