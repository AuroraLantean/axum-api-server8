use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};
use serde::Serialize;

pub async fn middleware_general(request: Request, next: Next) -> impl IntoResponse {
  println!("middleware_general");
  let response = next.run(request).await; // next.run() must run, OR all other routes will be blocked!
  response
  //Response::new(Body::from("Hello World!"))
}

#[allow(dead_code)]
pub async fn auth(request: Request, next: Next) -> impl IntoResponse {
  println!("auth");
  let response = next.run(request).await;
  response
}

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
