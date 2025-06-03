use async_graphql::{Object, http::GraphiQLSource};
use axum::response::{Html, IntoResponse};

pub struct Query;

#[Object]
impl Query {
  async fn graphql_method(&self) -> String {
    "graphql_method".to_owned()
  }
}

// handler for GraphQL
pub async fn graphql_handler() -> impl IntoResponse {
  Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
/*the endpoint above must match that in the router endpoint
Go to that endpoint, then a GraphQL page should load. Then enter:

query {
  graphqlMethod
}
*/
