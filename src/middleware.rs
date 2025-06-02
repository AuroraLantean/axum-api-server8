use std::sync::Arc;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::JWT_KEY;

pub async fn middleware_general(request: Request, next: Next) -> impl IntoResponse {
  println!("middleware_general");
  let response = next.run(request).await; // next.run() must run, OR all other routes will be blocked!
  response
  //Response::new(Body::from("Hello World!"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  pub sub: String, //user name
  pub exp: u64,
}
pub async fn auth(mut req: Request, next: Next) -> impl IntoResponse {
  println!("auth");
  match req.headers().get("authorization") {
    None => (StatusCode::UNAUTHORIZED, "No JWT").into_response(),
    Some(header_value) => {
      let token = header_value.to_str().unwrap();
      match decode(
        token,
        &DecodingKey::from_secret(JWT_KEY.as_bytes()),
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
pub struct JwtData {
  id: u32,
  name: String,
}
pub async fn extension_middleware(mut request: Request, next: Next) -> impl IntoResponse {
  println!("auth");
  let jwt_data = JwtData {
    id: 0,
    name: String::from("JohnDoe"),
  };
  request.extensions_mut().insert(Arc::new(jwt_data)); //need Arc for the clone trait requirement
  let response = next.run(request).await;
  response
}
