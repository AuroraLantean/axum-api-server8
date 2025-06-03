use std::sync::Arc;

use axum::{
  extract::{Request, State},
  http::StatusCode,
  middleware::Next,
  response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::SharedState;

pub async fn middleware_general(req: Request, next: Next) -> impl IntoResponse {
  println!("middleware_general");
  let response = next.run(req).await; // next.run() must run, OR all other routes will be blocked!
  response
  //Response::new(Body::from("Hello World!"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  pub sub: String, //user name
  pub exp: u64,
}
pub async fn auth(mut req: Request, next: Next) -> impl IntoResponse {
  println!("middleware: auth");
  let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET not found in .env");

  match req.headers().get("authorization") {
    None => (StatusCode::UNAUTHORIZED, "No JWT").into_response(),
    Some(header_value) => {
      let token = header_value.to_str().unwrap();
      match decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
      ) {
        Err(err) => (StatusCode::UNAUTHORIZED, err.to_string()).into_response(),
        Ok(token_data) => {
          let claims: JwtClaims = token_data.claims;
          let name = claims.sub;
          req.extensions_mut().insert(name);

          //call the handler
          let response = next.run(req).await;
          response
        }
      }
    }
  }
} //"into_response()" is useful when the match needs to return different types: tuple and response.

#[derive(Debug, Serialize)]
pub struct MiddlewareData {
  num: u32,
  str: String,
}
//use extensions to add data in middleware
pub async fn add_middleware_data(mut req: Request, next: Next) -> impl IntoResponse {
  println!("middleware: add_middleware_data");
  let jwt_data = MiddlewareData {
    num: 0,
    str: String::from("data from middleware"),
  };
  req.extensions_mut().insert(Arc::new(jwt_data)); //need Arc for the clone trait requirement
  let response = next.run(req).await;
  response
}

pub async fn get_state_in_middleware(
  State(state): State<Arc<SharedState>>,
  req: Request,
  next: Next,
) -> impl IntoResponse {
  println!("middleware: get_state_in_middleware. state: {:?}", state);
  let response = next.run(req).await;
  response
}
