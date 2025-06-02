use axum::{
  Extension, Json,
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use bcrypt;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;
use std::{
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};
use tokio_postgres::Client;

use crate::{JWT_KEY, middleware::JwtClaims, model::User};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FromUser {
  pub name: String,
  pub password: String,
  pub occupation: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
} //in postman: Body > raw: {...}
pub async fn add_user(
  State(client): State<Arc<Client>>,
  Json(input): Json<FromUser>,
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
    email: input.email.unwrap_or("".to_owned()),
    phone: input.phone.unwrap_or("".to_owned()),,
  };
  println!("{:?}", user);
  db.write().unwrap().insert(user.id, user.clone());*/
  (StatusCode::CREATED, "Success") //Code = `201 Created`
}
//docker exec -it postgres1 psql -U postgres
//\c db_name1
//SELECT * FROM db_name1;

pub async fn login(
  State(client): State<Arc<Client>>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("login: {:?}", input);
  let rows = client
    .query("SELECT * FROM users WHERE name = $1", &[&input.name])
    .await
    .unwrap();

  let hash_pw: String = rows[0].get(2); //password is at index 2
  let is_valid = bcrypt::verify(input.password, &hash_pw).unwrap();

  if is_valid {
    let name = rows[0].get(1);
    let jwt_claim = JwtClaims {
      sub: name,
      exp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60 * 60, // 1 hour valid JWT
    };
    let jwt_token = encode(
      &Header::default(),
      &jwt_claim,
      &EncodingKey::from_secret(JWT_KEY.as_bytes()),
    )
    .expect("JWT encode()");
    (
      StatusCode::OK,
      Json(json!({
        "result": "success",
        "jwt_token": jwt_token,
      })),
    )
  } else {
    (
      StatusCode::UNAUTHORIZED,
      Json(json!({
        "result": "Invalid password",
        "jwt_token": "",
      })),
    )
  }
}

// to receive request from auth middleware
pub async fn protected(Extension(name): Extension<String>) -> impl IntoResponse {
  let res = format!("Hello {}", name);
  (StatusCode::OK, res)
}

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
  Json(input): Json<FromUser>,
) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id"),
    name: input.name,
    password: input.password,
    occupation: String::from("developer"),
    email: input.email.unwrap_or("".to_owned()),
    phone: input.phone.unwrap_or("".to_owned()),
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

/*
//#[axum::debug]
#[allow(dead_code)]
pub async fn post_shared_state(
  State(shared_state): State<Arc<Mutex<SharedState>>>,
) -> impl IntoResponse {
  let mut state = shared_state.lock().unwrap();
  println!("input state: {:?}", state);
  (*state).token = "xyz".to_owned();
  println!("output state: {:?}", state);
  (StatusCode::OK, state.token.clone()) //Json(state.clone())
}
#[allow(dead_code)]
pub async fn get_shared_state(
  State(shared_state): State<Arc<Mutex<SharedState>>>,
) -> impl IntoResponse {
  //(StatusCode, Json<Arc<Mutex<SharedState>>>)
  let state = shared_state.lock().unwrap();
  println!("shared_state: {:?}", shared_state);
  (StatusCode::OK, state.token.clone()) //Json(shared_state)
}*/
