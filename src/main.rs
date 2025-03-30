use axum::{
  Json, Router,
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::{delete, get, patch, post, put},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  let endpoint = "0.0.0.0:3000";

  let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
  println!("server running on {endpoint:?}");
  axum::serve(listener, router()).await.unwrap();
}

fn router() -> Router {
  Router::new()
    .route("/", get(root))
    .route("/", post(post_handler))
    .route("/users", post(create_user))
}
async fn root() -> &'static str {
  "Hello Root!"
}

async fn post_handler() -> Response {
  Response::builder()
    .status(StatusCode::CREATED)
    .header("Content-Type", "application/json")
    .body(Body::from(r#"{"name":"john"}"#))
    .unwrap()
  //(StatusCode::CREATED, "New Post Added!")
} //[("Content-Type":"application/json")], r#"{"name":"john"}"#,

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
  let user = User {
    id: 1337, //id: Uuid::new_v4(),
    username: payload.username,
  };
  //db.write().unwrap().insert(user.id, user.clone());
  (StatusCode::CREATED, Json(user)) //Code = `201 Created`
}

// the input to our `create_user` handler
#[derive(Debug, Deserialize)]
struct CreateUser {
  username: String,
}
#[derive(Debug, Serialize, Clone)]
struct User {
  id: u64,
  //id: Uuid,
  username: String,
}
