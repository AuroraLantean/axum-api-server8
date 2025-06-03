use axum::{
  Extension, Json,
  extract::{Path, Query, State},
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

use crate::{middleware::JwtClaims, model::User};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FromUser {
  pub name: String,
  pub password: Option<String>,
  pub occupation: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
} //in postman: Body > raw: {...}
pub async fn add_user(
  State(client): State<Arc<Client>>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("add_user: {:?}", input);
  if None == input.password {
    (StatusCode::BAD_REQUEST, "empty password");
  }
  let hashed_result = bcrypt::hash(input.password.unwrap(), 10);
  let hashed_pw = if let Ok(hashed_pw) = hashed_result {
    hashed_pw
  } else {
    return (StatusCode::INTERNAL_SERVER_ERROR, "bcrypt hash error");
  };
  println!("hashed_pw: {:?}", hashed_pw);

  let query_result = client
    .execute(
      "INSERT INTO users (name, password) VALUES ($1, $2)",
      &[&input.name, &hashed_pw],
    )
    .await;

  if let Ok(_row_num) = query_result {
    (StatusCode::CREATED, "Success") //Code = `201 Created`
  } else {
    (StatusCode::INTERNAL_SERVER_ERROR, "db insertion error")
  }
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
}
//docker exec -it postgres1 psql -U postgres
//\c db_name1
//SELECT * FROM db_name1;

pub async fn login(
  State(client): State<Arc<Client>>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("login: {:?}", input);
  let query_result = client
    .query("SELECT * FROM users WHERE name = $1", &[&input.name])
    .await;
  let rows = if let Ok(rows) = query_result {
    rows
  } else {
    return (StatusCode::INTERNAL_SERVER_ERROR, "db query error").into_response();
  };

  if rows.len() == 0 {
    return (StatusCode::BAD_REQUEST, "user not found").into_response();
  } else if rows.len() > 1 {
    return (StatusCode::BAD_REQUEST, "multiple user found").into_response();
  }
  let hash_pw: String = rows[0].get(2); //password is at index 2
  let is_valid = if let Some(password) = input.password {
    let result = bcrypt::verify(password, &hash_pw);
    if let Ok(boo) = result {
      boo
    } else {
      return (StatusCode::BAD_REQUEST, "invalid password").into_response();
    }
  } else {
    return (StatusCode::BAD_REQUEST, "empty password").into_response();
  };
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
    let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET not found in .env");

    let jwt_result = encode(
      &Header::default(),
      &jwt_claim,
      &EncodingKey::from_secret(jwt_secret.as_bytes()),
    );
    let jwt_token = if let Ok(jwt_token) = jwt_result {
      jwt_token
    } else {
      return (StatusCode::INTERNAL_SERVER_ERROR, "jwt encoding error").into_response();
    };
    (
      StatusCode::OK,
      Json(json!({
        "result": "success",
        "jwt_token": jwt_token,
      })),
    )
      .into_response()
  } else {
    (StatusCode::UNAUTHORIZED, "Invalid password").into_response()
  }
}

// to receive request from auth middleware
pub async fn protected(Extension(name): Extension<String>) -> impl IntoResponse {
  let res = format!("Hello {}", name);
  (StatusCode::OK, res)
}

pub async fn get_user(Path(id): Path<String>) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id i32"),
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
  /*let users = users
      .values()
      .skip(pagination.offset.unwrap_or(0))
      .take(pagination.limit.unwrap_or(usize::MAX))
      .cloned()
      .collect::<Vec<_>>();
  Json(users)*/
}

pub async fn put_user(
  Path(id): Path<String>,
  Json(input): Json<FromUser>,
) -> (StatusCode, Json<User>) {
  let user = User {
    id: id.parse::<i32>().expect("id i32"),
    name: input.name,
    password: input.password.unwrap_or("".to_owned()),
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
    id: id.parse::<i32>().expect("id i32"),
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
    id: id.parse::<i32>().expect("id i32"),
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
