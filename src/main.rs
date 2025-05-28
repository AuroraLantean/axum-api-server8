use axum::{
  Router,
  routing::{get, post},
};
//use uuid::Uuid;
mod handlers;
use handlers::{
  add_user, custom_extractor, custom_extractor2, customized_path, delete_user, get_user,
  html_hello, internal_error, post_raw1, query_users, root, update_user,
};
mod model;

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
    .route("/hello", get(html_hello))
    .route("/users/{user_id}/teams/{team_id}", get(customized_path))
    .route("/", post(post_raw1))
    .route("/users", get(query_users).post(add_user))
    .route(
      "/users/{id}",
      get(get_user).patch(update_user).delete(delete_user),
    )
    .route("/custom_extractor", post(custom_extractor))
    .route("/custom_extractor2", post(custom_extractor2))
    .route("/internal_error", get(internal_error))
} // PUT method is to replace/add the entire resource
