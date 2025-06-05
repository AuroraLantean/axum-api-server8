use crate::{
  database::SeaPool,
  middleware::JwtClaims,
  model::{ActiveModel as UserActiveModel, Entity as UserEntity, Model, UserRaw},
};
use axum::{
  Extension, Json, debug_handler,
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
};
use bcrypt;
use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rust_decimal::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait};
use serde::Deserialize;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use time::{OffsetDateTime, UtcOffset};
use tokio_postgres::GenericClient;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FromUser {
  name: String,
  password: Option<String>,
  email: String,
  occupation: Option<String>,
  phone: Option<String>,
} //in postman: Body > raw: {...}
pub async fn add_user(
  State(pool): State<SeaPool>,
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

  let user1 = UserActiveModel {
    name: Set(input.name),
    password: Set(hashed_pw),
    email: Set(input.email),
    level: Set(1),
    ..Default::default()
  }; //id: Uuid::new_v4().to_string(),
  //balance: Decimal::from_str(&co);
  //balance: dec!(0),
  //updated_at: OffsetDateTime::now_utc(),
  //println!("{:?}", user1);

  let result = user1.insert(&pool).await;

  //let result = pool.get().await; //.map_err(internal_error)?;
  if let Ok(_) = result {
    (StatusCode::CREATED, "Success") //Code = `201 Created`
  } else {
    (StatusCode::INTERNAL_SERVER_ERROR, "db insert")
  }
}

//Must have all param fields or it will fail!
pub async fn add_with_query_params(
  State(_client): State<SeaPool>,
  Query(mut user): Query<UserRaw>,
) -> impl IntoResponse {
  println!("add_with_query_params: {:?}", user);
  user.occupation = Some("what job?".to_owned());
  (StatusCode::CREATED, Json(user)) //consumes the request body!
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Referal {
  firstname: String,
  lastname: String,
}
//Must have all param fields or it will fail!
pub async fn add_with_query_params2(
  State(_client): State<SeaPool>,
  Query(mut user): Query<UserRaw>,
  Query(referal): Query<Referal>,
) -> impl IntoResponse {
  println!("add_with_query_params: {:?}, referal: {:?}", user, referal);
  user.occupation = Some("what job?".to_owned());
  (StatusCode::CREATED, Json(user)) //consumes the request body!
}
#[axum::debug_handler]
pub async fn get_user_by_id(
  State(pool): State<SeaPool>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  println!("get_user_by_id");
  let id_i32 = id.parse::<i32>().expect("parse id to i32");
  let user_option = UserEntity::find_by_id(id_i32).one(&pool).await.unwrap();
  println!("user: {:?}", user_option);
  if let Some(user) = user_option {
    (StatusCode::OK, Json(user)).into_response()
  } else {
    (StatusCode::OK, "None found").into_response()
  }
}
pub async fn get_users(State(pool): State<SeaPool>) -> impl IntoResponse {
  println!("get_users");
  let all_users = UserEntity::find().all(&pool).await.unwrap();
  println!("all_users: {:?}", all_users);

  (StatusCode::OK, "all_users").into_response()

  /*let conn_result = pool.get().await; //.map_err(internal_error);
  let query_result = if let Ok(conn) = conn_result {
    let query_result = conn.query("SELECT * FROM users", &[]).await;
    query_result
  } else {
    return (StatusCode::INTERNAL_SERVER_ERROR, "db pool error").into_response();
  };

  let rows = if let Ok(rows) = query_result {
    rows
  } else {
    return (StatusCode::INTERNAL_SERVER_ERROR, "db query error").into_response();
  };
  //println!("rows: {:?}", rows);
  //for row in client.query("SELECT * FROM mytable", &[])? { ... }
  let row = &rows[0];
  println!("row: {:?}", row);
  let id: i32 = row.get(0); //Serial
  println!("id: {}", id);

  let name: String = row.get(1); //VARCHAR(n)
  println!("name: {}", name);
  let priority: i32 = row.try_get(6).unwrap_or(-1); //INT
  println!("priority: {}", priority);

  let balc: Decimal = row.try_get(7).unwrap_or(dec!(-1)); //Numeric(p,s)
  println!("balc: {:?}", balc);

  let time: SystemTime = row.try_get(8).unwrap(); //TIMESTAMPTZ
  println!("time: {:?}", time.elapsed());
  //let offset = UtcOffset::local_offset_at(row.get(8));

  //let time2: OffsetDateTime = row.get::<usize, OffsetDateTime>(8);
  //let offset = UtcOffset::local_offset_at(now)?;

  //let time3: chrono::DateTime<Utc> = row.get::<usize, DateTime<Utc>>(8);

  return (StatusCode::OK, "ok").into_response();
  */
  /*https://docs.rs/postgres/latest/postgres/types/trait.FromSql.html
  trait bound `: FromSql<'_>` is not satisfied{}
  */
}

pub async fn login(State(pool): State<SeaPool>, Json(input): Json<FromUser>) -> impl IntoResponse {
  println!("login: {:?}", input);
  /*let conn_result = pool.get().await; //.map_err(internal_error);

  let query_result = if let Ok(conn) = conn_result {
    let query_result = conn
      .query("SELECT * FROM users WHERE name = $1", &[&input.name])
      .await;
    query_result
  } else {
    return (StatusCode::INTERNAL_SERVER_ERROR, "db pool error").into_response();
  };

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
  }*/
  (StatusCode::OK, "TODO")
}

// to receive request from auth middleware
pub async fn protected(Extension(name): Extension<String>) -> impl IntoResponse {
  let res = format!("Hello {}", name);
  (StatusCode::OK, res)
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
  //#[serde(default, deserialize_with = "empty_string_as_none")]
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}
//{{host}}/users?offset=1&limit=100
pub async fn query_with_pagination(pagination: Query<Pagination> /*, State(db): State<Db> */) {
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
) -> (StatusCode, Json<UserRaw>) {
  let user = UserRaw {
    id: id.parse::<i32>().expect("id i32"),
    name: input.name,
    password: input.password.unwrap_or("".to_owned()),
    occupation: Some(String::from("developer")),
    email: input.email,
    phone: Some(input.phone.unwrap_or("".to_owned())),
    priority: Some(0),
    balance: dec!(0),
    updated_at: OffsetDateTime::now_utc(),
  };
  println!("new user: {:?}", user);
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::OK, Json(user)) //Code = `201 Created`
}

#[derive(Debug, Deserialize)]
pub struct PatchUser {
  name: Option<String>,
  occupation: Option<String>,
  email: Option<String>,
  phone: Option<String>,
}
pub async fn patch_user(
  Path(id): Path<String>,
  Json(input): Json<PatchUser>,
) -> (StatusCode, Json<UserRaw>) {
  let mut user = UserRaw {
    id: id.parse::<i32>().expect("id i32"),
    name: String::from("JohnDoe"),
    password: String::from("password"),
    occupation: Some(String::from("developer")),
    email: String::from("john@crypto.com"),
    phone: Some(String::from("1234")),
    priority: Some(0),
    balance: dec!(0),
    updated_at: OffsetDateTime::now_utc(),
  };
  println!("old user: {:?}", user);
  if let Some(name) = input.name {
    user.name = name;
  }
  if let Some(occupation) = input.occupation {
    user.occupation = Some(occupation);
  }
  if let Some(email) = input.email {
    user.email = email;
  }
  if let Some(phone) = input.phone {
    user.phone = Some(phone);
  }
  /*if let Some(balance) = input.balance {
    user.balance = i32::try_from(balance).expect("msg");
  }*/
  println!("new user: {:?}", user);
  (StatusCode::OK, Json(user))
}
pub async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<UserRaw>) {
  let user = UserRaw {
    id: id.parse::<i32>().expect("id i32"),
    name: String::from("JohnDoe"),
    password: String::from("password"),
    occupation: Some(String::from("developer")),
    email: String::from("john@crypto.com"),
    phone: Some(String::from("1234")),
    priority: Some(0),
    balance: dec!(0),
    updated_at: OffsetDateTime::now_utc(),
  };
  println!("old user: {:?}", user);

  /*if db.write().unwrap().remove(&id).is_some() {
    StatusCode::NO_CONTENT
  } else {
      StatusCode::NOT_FOUND
  }*/
  (StatusCode::FOUND, Json(user)) //Code = `201 Created`
}
