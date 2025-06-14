use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
  Router,
  http::{
    HeaderValue,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  },
  middleware::{from_fn, from_fn_with_state},
  routing::{delete, get, post},
};
//import generated trait containing gRPC methods that should be implemented for use with CalculatorServer
use crate::grpc::{
  AdminService, CalculatorService, GrpcState,
  proto::{self, admin_server::AdminServer},
};
use proto::calculator_server::CalculatorServer;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tonic::metadata::MetadataValue;
use tonic::service::Routes;
use tonic::{Request, Status};
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
mod users;
use users::*;
mod graphql;
use graphql::*;
mod entities;
mod grpc;
mod utils;
/*In axum 0.8 changes
  :id  => {id}

TODO: see example at Axum examples/anyhow for errors, and error-handling ...
See JWT example to make your own error
*/
//---------== gRPC Interceptor
// no async... for more advanced middleware, check tower
fn check_grpc_header(req: Request<()>) -> Result<Request<()>, Status> {
  let token: MetadataValue<_> = "Bearer secret-token".parse().unwrap();
  match req.metadata().get("authorization") {
    Some(t) if token == t => Ok(req),
    _ => Err(Status::unauthenticated("no valid auth token")),
  }
}
//---------== DO NOT SERIALIZE/DESERIALIZE REST AppSTATE!  //#[derive(Clone, FromRef)]
pub struct AppState {
  dbp: DatabaseConnection,
  msg: String,
  num: i32,
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();
  //tracing_subscriber::fmt::init();
  dotenvy::dotenv().expect(".env file not found");

  let server_addr = dotenvy::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

  let listener = tokio::net::TcpListener::bind(&server_addr)
    .await
    .expect("Could not add tcp listener");
  println!("server running on {server_addr:?}");

  let pool = sea_orm_db().await.unwrap();
  //let pool = tokio_postgres1().await;
  //let pool = sqlx_postgres1().await;
  //routes = router(pool)
  let rest_routes = router(Arc::new(AppState {
    dbp: pool.clone(),
    msg: "msg".to_owned(),
    num: 0,
  }));

  //make gRPC services share the same state
  let state = GrpcState::default();
  let calc = CalculatorService {
    state: state.clone(),
  };
  let admin = AdminService {
    state: state.clone(),
  };
  //let calculator = CalculatorService::default();
  //let admin = AdminService::default();

  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()
    .unwrap();

  //https://docs.rs/tonic/latest/tonic/service/struct.Routes.html
  let mut grpc_builder = Routes::builder();
  grpc_builder.add_service(reflection_service);
  grpc_builder.add_service(CalculatorServer::new(calc));
  grpc_builder.add_service(AdminServer::with_interceptor(admin, check_grpc_header));
  let grpc_routes = grpc_builder.routes().into_axum_router();
  //TODO: add tonic-web: see in aarchive

  let cors_layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT])
    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap());
  //allow_method: [Method::GET, Method::POST]
  //allow)credentials(true)

  let routes = rest_routes.merge(grpc_routes).layer(cors_layer);

  axum::serve(listener, routes).await.unwrap();
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

fn router(app_state: Arc<AppState>) -> Router {
  //pool: DatabaseConnection
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
  let shared_user = SharedUser {
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
    .route("/get_all_users", get(get_all_users))
    .route("/login", post(login))
    .route("/protected", get(protected).layer(from_fn(auth)))
    .route("/protected2", get(protected2).layer(from_fn(auth)))
    .route("/add_with_query_params", post(add_with_query_params))
    .route("/add_with_query_params2", post(add_with_query_params2))
    .route("/user_by_name/{:name}", get(get_user_by_name))
    .route(
      "/users/{id}",
      get(get_user_by_id)
        .put(put_user)
        .patch(patch_user)
        .delete(delete_user),
    )
    .route(
      "/delete_partial_name_users",
      delete(delete_partial_name_users),
    )
    .route("/delete_all_users", delete(delete_all_users))
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
    .route(
      "/add_middleware_data",
      get(middleware_data_handler).route_layer(from_fn(add_middleware_data)),
    )
    .nest("/user", user_router)
    .nest_service("/static", static_files)
    //.add_service(calculator)
    //.nest_service("/calculator", calculator)
    //.into_axum_router()
    .route("/wildcard/{*rest}", get(wildcard_handler))
    .route("/uri/xyz", get(uri_handler))
    .route("/contact_form", post(contact_form_handler))
    .route(
      "/graphql",
      get(graphql_handler).post_service(GraphQL::new(schema)),
    )
    .merge(route1)
    //.fallback(fallback_handler)
    .layer(from_fn(middleware_general))
    .with_state(app_state)
    .route("/get_state_in_handler", get(get_state_handler))
    .with_state((shared_state, shared_user))
    .route(
      "/mut_shared_state",
      get(get_mut_shared_state_handler).post(post_mut_shared_state_handler),
    )
    .with_state(arc_shared_state_mut)
  //.layer(cors_layer)

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

    Tokio-posgres: To be safe from sql injection, make sure that all user data is passed in the second params argument.

    Rust <=> Postgres Types
    https://docs.rs/postgres/latest/postgres/types/trait.FromSql.html
    https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html
    https://docs.rs/sea-orm/latest/sea_orm/entity/enum.ColumnType.html
    Serial, INT4, INT <=> i32
    BOOL <=> bool
    OID  <=> u32
    INT8 <=> i64
    FLOAT4 <=> f32
    FLOAT8 <=> f64
    VARCHAR <=> String, &str
    Numeric <=> rust_decimal::Decimal, or bigdecimal::BigDecimal (those crates are included in SeaORM features)
    timetz  <=> DateTimeWithTimeZone from chrono::DateTime<FixedOffset>, included in SeaORM features

    https://github.com/sfackler/rust-postgres/blob/c5ff8cfd86e897b7c197f52684a37a4f17cecb75/postgres-types/src/lib.rs#L727


    ------== Session
    let session_store = session(pool).await;
    router(session_store);

    in router(session_store: SessionStore<SessionSqlitePool>) { ...
    Router::new().route("/session_set_handler", get(session_set_handler)).layer(SessionLayer::new(session_store))
    }*/
}
