use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::entities::users::ActiveModel;
use crate::{
  AppState,
  entities::{prelude::*, *},
  middleware::JwtClaims,
  model::UserCopied,
};
use axum::{
  Extension, Json,
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
};
use bcrypt;
use jsonwebtoken::{EncodingKey, Header, encode};
use rust_decimal::prelude::*;
use sea_orm::*;
use serde::Deserialize;
use serde_json::json;

//https://www.sea-ql.org/sea-orm-tutorial/ch01-05-basic-crud-operations.html
//https://www.sea-ql.org/sea-orm-tutorial/ch01-08-sql-with-sea-query.html
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FromUser {
  name: String,
  password: Option<String>,
  email: Option<String>,
  occupation: Option<String>,
  phone: Option<String>,
  level: Option<i32>,
  balance: Option<Decimal>,
} //in postman: Body > raw: {...}
pub async fn add_user(
  State(state): State<Arc<AppState>>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("add_user: {:?}", input);
  let email = if let Some(email) = input.email {
    email
  } else {
    return (StatusCode::BAD_REQUEST, "email invalid");
  };
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

  //https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/#insert-one
  let user1 = ActiveModel {
    name: Set(input.name),
    password: Set(hashed_pw),
    email: Set(email),
    level: Set(0),
    ..Default::default()
  }; //ActiveValue::NotSet,
  //id: Uuid::new_v4().to_string(),
  //balance: Decimal::from_str(&balc_str);
  //balance: dec!(0),
  //updated_at: OffsetDateTime::now_utc(),

  let result = user1.insert(&state.dbp).await;
  //("INSERT INTO users (name, password) VALUES ($1, $2) RETURNING *", name, password)

  match result {
    Ok(model) => {
      println!("model: {:?}", model);
      (StatusCode::CREATED, "Success")
    }
    Err(err) => {
      eprintln!("DB error: {}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, "db insert")
    }
  }
}

//Must have all param fields or it will fail!
pub async fn add_with_query_params(
  State(_client): State<Arc<AppState>>,
  Query(mut user): Query<UserCopied>,
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
  State(_client): State<Arc<AppState>>,
  Query(mut user): Query<UserCopied>,
  Query(referal): Query<Referal>,
) -> impl IntoResponse {
  println!("add_with_query_params: {:?}, referal: {:?}", user, referal);
  user.occupation = Some("what job?".to_owned());
  (StatusCode::CREATED, Json(user)) //consumes the request body!
}

#[axum::debug_handler]
pub async fn get_user_by_id(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  //Result<impl IntoResponse,(StatusCode, Json<Value>)
  println!("get_user_by_id");
  let id_i32 = id.parse::<i32>().expect("parse id to i32");
  let user_option = Users::find_by_id(id_i32)
    .into_json()
    .one(&state.dbp)
    .await
    .expect("find_by_id");
  println!("user: {:?}", user_option);
  if let Some(user) = user_option {
    println!("user: {:?}", user);
    //serde_json::to_string_pretty(&user).unwrap()
    (StatusCode::OK, Json(user)).into_response()
  } else {
    (StatusCode::OK, "None found").into_response()
  }
}
pub async fn get_user_by_name(
  State(state): State<Arc<AppState>>,
  Path(name): Path<String>,
) -> impl IntoResponse {
  //Result<impl IntoResponse,(StatusCode, Json<Value>)
  println!("get_user_by_name");
  let user_option = Users::find()
    .filter(users::Column::Name.eq(name))
    .into_json()
    .one(&state.dbp)
    .await
    .expect("get_user_by_name");
  println!("user: {:?}", user_option);
  if let Some(user) = user_option {
    println!("user: {:?}", user);
    (StatusCode::OK, Json(user)).into_response()
  } else {
    (StatusCode::OK, "None found").into_response()
  }
}
#[axum::debug_handler]
pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
  //the trait bound `Vec<serde_json::Value>: IntoResponse` is not satisfied
  println!("get_users");
  println!("state: {} {}", state.msg, state.num);
  let result = Users::find().into_json().all(&state.dbp).await;

  match result {
    Ok(users) => {
      println!("users: {:?}", users);
      let json_str = serde_json::to_string_pretty(&users).expect("serde_json::to_string");
      (StatusCode::OK, json_str)
    }
    Err(err) => {
      println!("err: {:?}", err);
      (StatusCode::OK, err.to_string())
    }
  }
}

pub async fn login(
  State(state): State<Arc<AppState>>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("login: {:?}", input);

  let user_option = Users::find()
    .filter(users::Column::Name.eq(input.name))
    .one(&state.dbp)
    .await
    .expect("get_user_by_name"); //No JSON!

  let user = if let Some(user) = user_option {
    println!("user: {:?}", user);
    user
  } else {
    return (StatusCode::OK, "None found").into_response();
  };
  println!("user: {:?}", user);
  let hash_pw: String = user.password;
  //(StatusCode::OK, "").into_response()
  //let hash_pw: String = rows[0].get(2); //password is at index 2

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
    let name = user.name; // rows[0].get(1);
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

//https://www.sea-ql.org/SeaORM/docs/basic-crud/select/#paginate-result
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
  //#[serde(default, deserialize_with = "empty_string_as_none")]
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}
//{{host}}/users?offset=1&limit=100
pub async fn query_with_pagination(
  State(state): State<Arc<AppState>>,
  pagination: Query<Pagination>, /*, State(db): State<Db> */
) {
  println!("pagination: {:?} {:?}", pagination.offset, pagination.limit);
  let limit = pagination.limit.unwrap_or(10);
  let _offset = pagination.offset.unwrap_or(0) * limit;

  let mut pages: Paginator<_, _> = Users::find()
    .filter(users::Column::Name.contains("John"))
    .order_by_asc(users::Column::Name)
    .into_json()
    .paginate(&state.dbp, 50);

  while let Some(user) = pages.fetch_and_next().await.unwrap() {
    println!("user: {:?}", user);
    //Vec<serde_json::Value>
  }

  //let _query_result = "SELECT * FROM users ORDER by id LIMIT $1 OFFSET $2";//limit as i32, offset as i32
  /*let users = users
      .values()
      .skip(pagination.offset.unwrap_or(0))
      .take(pagination.limit.unwrap_or(usize::MAX))
      .cloned()
      .collect::<Vec<_>>();
  Json(users)*/
}

#[derive(Debug, Deserialize)]
pub struct PatchUser {
  email: Option<String>,
  occupation: Option<String>,
  phone: Option<String>,
  balance: Option<String>,
}
pub async fn patch_user(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
  Json(input): Json<PatchUser>,
) -> impl IntoResponse {
  println!("patch_user: {:?}", input);
  let id_i32 = id.parse::<i32>().expect("parse id to i32");

  let user_option = Users::find_by_id(id_i32).one(&state.dbp).await.unwrap();
  //("UPDATE users SET password = $1, email = $2 WHERE id = $3 RETURNING *", password, email, id)

  //https://www.sea-ql.org/SeaORM/docs/basic-crud/update/ ... convert from Model into an ActiveModel first.
  let mut user: ActiveModel = if let Some(user) = user_option {
    user.into()
  } else {
    return (StatusCode::OK, "None found").into_response();
  };

  println!("old user: {:?}", user);
  if let Some(occupation) = input.occupation {
    user.occupation = Set(Some(occupation));
  }
  if let Some(email) = input.email {
    user.email = Set(email);
  }
  if let Some(phone) = input.phone {
    user.phone = Set(Some(phone));
  }
  if let Some(balc_str) = input.balance {
    let balance_dec = Decimal::from_str(balc_str.as_str()).expect("convert String to Decimal");
    user.balance = Set(balance_dec);
  }
  let result = user.update(&state.dbp).await;
  match result {
    Ok(new_model) => {
      println!("new user: {:?}", new_model);
      (StatusCode::OK, "success").into_response()
    }
    Err(err) => {
      eprintln!("DB error: {}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
    }
  }
}

pub async fn delete_user(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let id = id.parse::<i32>().expect("id i32");
  let user = ActiveModel {
    id: ActiveValue::Set(id),
    ..Default::default()
  };
  let result = user.delete(&state.dbp).await;
  //("DELETE FROM users WHERE id = $1", id)
  //("DELETE FROM users *")
  match result {
    Ok(res) => {
      println!("rows_affected: {}", res.rows_affected);
      (StatusCode::OK, "success")
    }
    Err(err) => {
      eprintln!("DB error: {}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, "error")
    }
  }
}
pub async fn delete_many_users(
  State(state): State<Arc<AppState>>,
  Path(name): Path<String>,
) -> impl IntoResponse {
  let result = users::Entity::delete_many()
    .filter(users::Column::Name.contains(name))
    .exec(&state.dbp)
    .await;
  //("DELETE FROM users * WHERE name = $1")
  match result {
    Ok(res) => {
      println!("rows_affected: {}", res.rows_affected);
      (StatusCode::OK, "success")
    }
    Err(err) => {
      eprintln!("DB error: {}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, "error")
    }
  }
}

//Use patch_user instead, bcos you should fetch to see if the user exist first!
pub async fn put_user(
  State(_state): State<Arc<AppState>>,
  Path(_id): Path<String>,
  Json(input): Json<FromUser>,
) -> impl IntoResponse {
  println!("put_user: {:?}", input);
  (StatusCode::OK, "TODO")
}
