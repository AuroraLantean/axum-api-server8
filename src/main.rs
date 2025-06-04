use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
  Router,
  http::{
    HeaderValue,
    header::{AUTHORIZATION, CONTENT_TYPE},
  },
  middleware::{from_fn, from_fn_with_state},
  routing::{get, post},
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;
use tower_http::{
  cors::{Any, CorsLayer},
  services::ServeDir,
};
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
mod graphql;
use graphql::*;

/*In axum 0.8 changes
  from :id to {id}
*/
#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  dotenvy::dotenv().expect(".env file not found");
  let endpoint = "0.0.0.0:3000";

  let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
  println!("server running on {endpoint:?}");

  let client = tokio_postgres1().await;
  axum::serve(listener, router(client)).await.unwrap();
}

#[derive(Debug, Clone, Serialize)]
struct SharedState {
  mesg: String,
  num: u32,
}
#[derive(Debug, Clone)]
struct SharedUser {
  name: String,
  age: u32,
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

  let cors_layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers([CONTENT_TYPE, AUTHORIZATION])
    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap());
  //allow_method: [Method::GET, Method::POST]

  let static_files = ServeDir::new("./assets"); // localhost:3000/static/bitcoin.jpg
  let arc_shared_state_mut = Arc::new(Mutex::new(SharedState {
    mesg: "arc_sharedState_mut".to_owned(),
    num: 100,
  }));
  let arc_shared_state = Arc::new(SharedState {
    mesg: "arc_sharedState".to_owned(),
    num: 100,
  });
  let shared_state = SharedState {
    mesg: "sharedState".to_owned(),
    num: 200,
  };
  let user = SharedUser {
    name: "sharedUser".to_owned(),
    age: 27,
  };

  let user_router: Router<_> = Router::new()
    .route("/profile", get(user_profile))
    .route("/setting", get(user_setting)); //nested route

  let route1 = Router::new().route("/about", get(about_handler)); //merged route

  let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

  //#[axum::debug_handler]
  Router::new()
    .route("/", get(root))
    .route(
      "/get_state_in_middleware",
      get(get_state_in_middleware_handler),
    )
    .layer(from_fn_with_state(
      arc_shared_state.clone(),
      get_state_in_middleware,
    ))
    .route("/text", get(|| async { "hello" }))
    .route("/redirect", get(redirect_handler))
    .route("/html", get(html))
    .route("/query_params", get(query_params))
    .route("/request_params", get(request_params).post(request_params))
    .route("/users", get(query_with_pagination).post(add_user))
    .route("/login", post(login))
    .route("/protected", get(protected).layer(from_fn(auth)))
    .route("/add_with_query_params", post(add_with_query_params))
    .route("/add_with_query_params2", post(add_with_query_params2))
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
    .route(
      "/into_response_trait_custom_output/{id}",
      get(into_response_trait_custom_output),
    )
    .route("/resp_output/{id}", get(resp_output))
    .route("/custom_extractor", post(custom_extractor))
    .route("/custom_extractor2", post(custom_extractor2))
    .route("/internal_error", get(internal_error))
    .route(
      "/add_middleware_data",
      get(middleware_data_handler).route_layer(from_fn(add_middleware_data)),
    )
    .nest("/user", user_router)
    .nest_service("/static", static_files)
    .route("/wildcard/{*rest}", get(wildcard_handler))
    .route("/uri/xyz", get(uri_handler))
    .route("/contact_form", post(contact_form_handler))
    .route(
      "/graphql",
      get(graphql_handler).post_service(GraphQL::new(schema)),
    )
    .merge(route1)
    .fallback(fallback_handler)
    .layer(from_fn(middleware_general))
    .with_state(Arc::new(client))
    .route("/get_state_in_handler", get(get_state_handler))
    .with_state((shared_state, user))
    .route(
      "/mut_shared_state",
      get(get_mut_shared_state_handler).post(post_mut_shared_state_handler),
    )
    .with_state(arc_shared_state_mut)
    .layer(cors_layer)

  /*.with_state() will cause all routes above it to receive that state. So you can have multiple states like:
  .route().with_state(client_in_arc)
  .route().with_state(user, message)
  .route().with_state(shared_state_in_arc)

    the layer middleware will run first, then the route_layer middleware will run second, then the route function runs.

    impl IntoResponse / into_response is useful for returning different types: tuple or Json.

    PUT method is to replace/add the entire resource

    #[debug_handler] is available in Axum macros feature

    Struct must have pub fields if it is used in another file

    Axum::Json() will consume the request body

    ------== Session
    let session_store = session(pool).await;
    router(session_store);

    in router(session_store: SessionStore<SessionSqlitePool>) { ...
    Router::new().route("/session_set_handler", get(session_set_handler)).layer(SessionLayer::new(session_store))
    }*/
}

//TODO:
