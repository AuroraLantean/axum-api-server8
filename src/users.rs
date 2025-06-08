use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::entities::users::ActiveModel;
use crate::utils::{PaginationOut, own};
use crate::{
  AppState,
  entities::{prelude::*, *},
  middleware::JwtClaims,
};
use axum::{
  Extension, Json,
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
};
use bcrypt;
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

//-----------== Sea ORM Model:
/*USE SeaORM CLI to generate the entity model first: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/

Then copy it to this file to make the UserInput struct below, then delete some fields like id, level, ...

Data Types conversion between Postgres and Rust https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/#column-type

borrowed_str = format!("Something went wrong: {}", self.0);
status_code = StatusCode::XYZ, XYZ = INTERNAL_SERVER_ERROR, CREATED, OK, FOUND, NO_CONTENT...

To return "impl IntoResponse":
(status_code, borrowed_str)
(status_code, Json(StructXyz))


(status_code, Json(Vec<users::Model))
(status_code, Json<JsonValue>), whose Json value = Json(json!({"result": "success"}))

if you need to return different types:
(status_code, borrowed_str).into_response()
response: Response<Body>
*/

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
  updated_at: Option<DateTimeWithTimeZone>,
} //in postman: Body > raw: {...}
#[axum::debug_handler]
pub async fn add_user(
  State(state): State<Arc<AppState>>,
  Json(input): Json<FromUser>,
) -> Result<String, (StatusCode, String)> {
  println!("add_user: {:?}", input);
  let email = if let Some(email) = input.email {
    email
  } else {
    return Err((StatusCode::BAD_REQUEST, own("email invalid")));
  };
  if None == input.password {
    return Err((StatusCode::BAD_REQUEST, own("password invalid")));
  }
  let hashed_result = bcrypt::hash(input.password.unwrap(), 10);
  let hashed_pw = if let Ok(hashed_pw) = hashed_result {
    hashed_pw
  } else {
    return Err((StatusCode::FAILED_DEPENDENCY, own("bcrypt hash error")));
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

  let model = user1
    .insert(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  println!("user added: {:?}", model);
  //("INSERT INTO users (name, password) VALUES ($1, $2) RETURNING *", name, password)
  //eprintln!("DB error: {}", err);
  Ok(format!("success added user id: {}", model.id))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserQueryParams {
  pub name: String,
  pub password: String,
  pub email: String,
  pub occupation: Option<String>,
  pub phone: Option<String>,
}
//Must have all param fields or it will fail!
pub async fn add_with_query_params(
  State(_client): State<Arc<AppState>>,
  Query(mut user): Query<UserQueryParams>,
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
  Query(mut user): Query<UserQueryParams>,
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
) -> Result<users::Model, (StatusCode, String)> {
  println!("get_user_by_id");
  let id_i32 = id.parse::<i32>();
  if id_i32.is_err() {
    return Err((StatusCode::BAD_REQUEST, own("parse id to i32")));
  };
  let user_option = Users::find_by_id(id_i32.unwrap())
    .one(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?; //.into_json()

  println!("user: {:?}", user_option);
  if let Some(user) = user_option {
    println!("user: {:?}", user);
    Ok(user)
  } else {
    Err((StatusCode::NOT_FOUND, own("Not found")))
  }
}

#[axum::debug_handler]
pub async fn get_user_by_name(
  State(state): State<Arc<AppState>>,
  Path(name): Path<String>,
) -> Result<users::Model, (StatusCode, String)> {
  println!("get_user_by_name");
  let user_option = Users::find()
    .filter(users::Column::Name.eq(name))
    .one(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?; //.into_json()
  println!("user: {:?}", user_option);

  if let Some(user) = user_option {
    println!("user: {:?}", user);
    Ok(user)
  } else {
    Err((StatusCode::NOT_FOUND, own("Not found")))
  }
}

#[axum::debug_handler]
pub async fn get_all_users(
  State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<users::Model>>, (StatusCode, String)> {
  // Json(json!({ "data": 42 })) : Json<Value>
  println!("get_all_users");
  println!("state: {} {}", state.msg, state.num);
  let result = Users::find()
    .all(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?; // .into_json()
  Ok(Json(result))
}

#[axum::debug_handler]
pub async fn login(
  State(state): State<Arc<AppState>>,
  Json(input): Json<FromUser>,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
  println!("login: {:?}", input);

  let user_option = Users::find()
    .filter(users::Column::Name.eq(input.name))
    .one(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?; //No JSON!

  let user = if let Some(user) = user_option {
    println!("user: {:?}", user);
    user
  } else {
    return Err((StatusCode::NOT_FOUND, own("not_found")));
  };
  println!("user: {:?}", user);
  let hash_pw: String = user.password;
  //let hash_pw: String = rows[0].get(2); //password is at index 2

  let is_valid = if let Some(password) = input.password {
    let result = bcrypt::verify(password, &hash_pw);
    if let Ok(boo) = result {
      boo
    } else {
      return Err((StatusCode::BAD_REQUEST, own("Invalid password")));
    }
  } else {
    return Err((StatusCode::BAD_REQUEST, own("Empty password")));
  };
  if is_valid {
    let name: String = user.name; // rows[0].get(1);
    let exp_time = SystemTime::now().duration_since(UNIX_EPOCH);
    if exp_time.is_err() {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, own("duration_since")));
    };
    let jwt_claim = JwtClaims {
      sub: name,
      id: user.id,
      exp: exp_time.unwrap().as_secs() + 60 * 60, // 1 hour valid JWT
    };
    let jwt_secret = dotenvy::var("JWT_SECRET");
    if jwt_secret.is_err() {
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        own("JWT_SECRET not found in .env"),
      ));
    }
    let jwt_result = encode(
      &Header::default(),
      &jwt_claim,
      &EncodingKey::from_secret(jwt_secret.unwrap().as_bytes()),
    );
    let jwt_token = if let Ok(jwt_token) = jwt_result {
      jwt_token
    } else {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, own("jwt encoding error")));
    };
    Ok(Json(json!({
      "result": "success",
      "id": user.id,
      "jwt_token": jwt_token,
    })))
  } else {
    Err((StatusCode::BAD_REQUEST, own("Invalid password")))
  }
}

// to receive request from auth middleware
#[axum::debug_handler]
pub async fn protected(Extension(name): Extension<String>) -> impl IntoResponse {
  let res = format!("Successful login for {}", name);
  (StatusCode::OK, res)
}
#[axum::debug_handler]
pub async fn protected2(Extension(id): Extension<i32>) -> impl IntoResponse {
  let res = format!("Successful login for {}", id);
  (StatusCode::OK, res)
}

//https://www.sea-ql.org/SeaORM/docs/basic-crud/select/#paginate-result
#[derive(Debug, Deserialize, Default)]
pub struct PaginationInput {
  //#[serde(default, deserialize_with = "empty_string_as_none")]
  //#[serde(with = "serde_with::rust::string_empty_as_none")]
  pub page: Option<u64>,
  pub amount: Option<u64>,
} //{{host}}/users?page=1&amount=10
#[axum::debug_handler]
pub async fn query_with_pagination(
  State(state): State<Arc<AppState>>,
  pagination: Query<PaginationInput>,
) -> Result<PaginationOut, (StatusCode, String)> {
  println!("pagination: {:?}", pagination);
  let amount = pagination.amount.unwrap_or(10);
  let page = pagination.page.unwrap_or(1);
  if page < 1 {
    return Err((StatusCode::BAD_REQUEST, own("page < 1")));
  };

  let paginator: Paginator<_, _> = Users::find()
    .order_by_asc(users::Column::Id)
    .paginate(&state.dbp, amount);
  //.filter(users::Column::Name.contains("John"))

  let num_pages = paginator
    .num_pages()
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  println!("num_pages: {}", num_pages);
  // .into_json()
  let out = paginator
    .fetch_page(page - 1)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?; //.map(|p| (p, num_pages))?;
  Ok(PaginationOut(out, num_pages))

  /*while let Some(users) = paginator.fetch_and_next().await? {
    println!("users: {:?}", users);
  }*/
  //let _query_result = "SELECT * FROM users ORDER by id LIMIT $1 OFFSET $2";//limit as i32, offset as i32
}

#[derive(Debug, Deserialize)]
pub struct PatchUser {
  email: Option<String>,
  occupation: Option<String>,
  phone: Option<String>,
  balance: Option<String>,
}
#[axum::debug_handler]
pub async fn patch_user(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
  Json(input): Json<PatchUser>,
) -> Result<users::Model, (StatusCode, String)> {
  println!("patch_user: {:?}", input);
  let id_i32 = id.parse::<i32>();
  if id_i32.is_err() {
    return Err((StatusCode::BAD_REQUEST, own("parse id to i32")));
  };
  let user_option = Users::find_by_id(id_i32.unwrap())
    .one(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  //("UPDATE users SET password = $1, email = $2 WHERE id = $3 RETURNING *", password, email, id)

  //https://www.sea-ql.org/SeaORM/docs/basic-crud/update/ ... convert from Model into an ActiveModel first.
  let mut user: ActiveModel = if let Some(user) = user_option {
    user.into()
  } else {
    return Err((StatusCode::NOT_FOUND, own("not_found")));
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
    let balance_dec = Decimal::try_from(balc_str.as_str());
    if balance_dec.is_err() {
      return Err((StatusCode::BAD_REQUEST, own("convert String to Decimal")));
    }
    user.balance = Set(balance_dec.unwrap());
  }
  let new_model = user
    .update(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  Ok(new_model)
}

#[axum::debug_handler]
pub async fn delete_user(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
  let id_i32 = id.parse::<i32>();
  if id_i32.is_err() {
    return Err((StatusCode::BAD_REQUEST, own("parse id to i32")));
  };
  let user_option = Users::find_by_id(id_i32.unwrap())
    .one(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  //.ok_or(Err(db_err("Not found")))
  //.map(Into::into)?; //.into_json()

  println!("user: {:?}", user_option);
  let user = if let Some(user) = user_option {
    println!("user: {:?}", user);
    user
  } else {
    return Err((StatusCode::NOT_FOUND, own("not_found")));
  };

  let result = user
    .delete(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  //("DELETE FROM users WHERE id = $1", id)
  //("DELETE FROM users *")
  //result.rows_affected
  Ok(format!("success. rows_affected: {}", result.rows_affected))
}

#[axum::debug_handler]
pub async fn delete_partial_name_users(
  State(state): State<Arc<AppState>>,
  Path(name): Path<String>,
) -> Result<String, (StatusCode, String)> {
  let result = users::Entity::delete_many()
    .filter(users::Column::Name.contains(name))
    .exec(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  //("DELETE FROM users * WHERE name = $1")
  Ok(format!("success. rows_affected: {}", result.rows_affected))
}

#[axum::debug_handler]
pub async fn delete_all_users(
  State(state): State<Arc<AppState>>,
) -> Result<String, (StatusCode, String)> {
  let result = users::Entity::delete_many()
    .exec(&state.dbp)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
  //("DELETE FROM users *")
  Ok(format!("success. rows_affected: {}", result.rows_affected))
}

//Use patch_user instead, bcos you should fetch to see if the user exist first!
#[axum::debug_handler]
pub async fn put_user(
  State(_state): State<Arc<AppState>>,
  Path(_id): Path<String>,
  Json(input): Json<FromUser>,
) -> Result<String, (StatusCode, String)> {
  println!("put_user: {:?}", input);
  Ok("success".to_owned())
}
