use axum::{
  Router,
  routing::{get, post},
};
//use uuid::Uuid;
mod handlers;
use handlers::{
  add_user, customized_path, delete_user, html_hello, post_raw1, query_users, read_user, root,
  update_user,
};

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
      get(read_user).patch(update_user).delete(delete_user),
    )
}
