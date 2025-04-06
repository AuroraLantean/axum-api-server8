use axum::{
  Json, Router,
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::{delete, get, patch, post, put},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod handlers;
use handlers::{add_user, delete_user, post_handler, read_user, root, update_user};

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
    .route("/users", post(add_user))
    .route(
      "/users/{id}",
      get(read_user).patch(update_user).delete(delete_user),
    )
}
