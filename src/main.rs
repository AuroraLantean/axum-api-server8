use axum::{
  Json, Router,
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::{delete, get, patch, post, put},
};
use serde::{Deserialize, Serialize};

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
    // `GET /` goes to `root`
    .route("/", get(root))
    .route("/", post(post_handler))
  //.route("/users", post(create_user))
}
// basic handler that responds with a static string
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

//#[derive(Serialize)]
struct User {
  id: u64,
  username: String,
}
/*async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
*/
