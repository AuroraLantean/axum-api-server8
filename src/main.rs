use axum::{
  Router,
  routing::{get, post},
};
//use uuid::Uuid;
mod handlers;
use handlers::{
  add_user, custom_extractor, custom_extractor2, customized_path, delete_user, dynamic_json_output,
  get_user, html, internal_error, patch_user, post_raw1, put_user, query_params, query_users,
  request_params, resp_output, root,
};
mod model;

/*In axum 0.8 changes
  from :id to {id}
*/
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
    .route("/text", get(|| async { "hello" }))
    .route("/html", get(html))
    .route("/query_params", get(query_params))
    .route("/request_params", get(request_params).post(request_params))
    .route("/users", get(query_users).post(add_user))
    .route(
      "/users/{id}",
      get(get_user)
        .put(put_user)
        .patch(patch_user)
        .delete(delete_user),
    )
    .route("/users/{user_id}/teams/{team_id}", get(customized_path))
    .route("/", post(post_raw1))
    .route("/dynamic_json_output/{id}", get(dynamic_json_output))
    .route("/resp_output/{id}", get(resp_output))
    .route("/custom_extractor", post(custom_extractor))
    .route("/custom_extractor2", post(custom_extractor2))
    .route("/internal_error", get(internal_error))
} // PUT method is to replace/add the entire resource
