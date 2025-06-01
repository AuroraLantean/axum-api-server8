use axum::{
  Router,
  middleware::from_fn,
  routing::{get, post},
};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;
//use uuid::Uuid;
mod handlers;
use handlers::*;
mod middleware;
use middleware::*;
mod database;
use database::*;
mod model;
mod users;
use users::*;

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

  let client = db().await;
  axum::serve(listener, router(client)).await.unwrap();
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SharedState {
  auth: String,
  token: String,
  //db_client: Arc<DbClient>,
  //db_pool: Arc<DbPool>,
  //config: Arc<Config>,
}

fn router(client: Client) -> Router {
  //TODO: https://docs.rs/deadpool-postgres/latest/deadpool_postgres/
  //TODO: https://github.com/tokio-rs/axum/discussions/2819
  //TODO: pass pool to handler: https://stackoverflow.com/questions/76246672/unable-to-pass-tokio-postgres-pool-connections-to-axum-handler
  //TODO: bb8_postgres pool: https://github.com/tokio-rs/axum/blob/main/examples/tokio-postgres/src/main.rs
  // Initialize the database connection pool and configuration information
  /*let db_client = Arc::new(DbClient { client });
    let db_pool = Arc::new(DbPool {});
    let config = Arc::new(Config {});
  */
  let _shared_state = Arc::new(Mutex::new(SharedState {
    auth: "1234".to_owned(),
    token: "abcd".to_owned(),
  }));

  let user_router: Router<_> = Router::new()
    .route("/profile", get(user_profile))
    .route("/setting", get(user_setting)); //nested route

  let route1 = Router::new().route("/about", get(about_handler)); //merged route

  //#[axum::debug_handler]
  Router::new()
    .route("/", get(root))
    .route("/text", get(|| async { "hello" }))
    .route("/redirect", get(redirect_handler))
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
    .route(
      "/into_response_trait_dynamic_output/{id}",
      get(into_response_trait_dynamic_output),
    )
    .route("/custom_extractor", post(custom_extractor))
    .route("/custom_extractor2", post(custom_extractor2))
    .route("/internal_error", get(internal_error))
    /*.route(
      "/shared_state",
      post(post_shared_state)
        .get(get_shared_state)
        .route_layer(from_fn(auth)),
    )*/
    .route(
      "/extension",
      get(extension_handler).route_layer(from_fn(extension_middleware)),
    )
    .nest("/user", user_router)
    .route("/wildcard/{*rest}", get(wildcard_handler))
    .route("/uri/xyz", get(uri_handler))
    .route("/contact_form", post(contact_form_handler))
    .merge(route1)
    .fallback(fallback_handler)
    .layer(from_fn(middleware_general))
    //.with_state(shared_state)
    .with_state(Arc::new(client))
  //the layer middleware will run first, then the route_layer middleware will run second, then the route function runs.
} // PUT method is to replace/add the entire resource
