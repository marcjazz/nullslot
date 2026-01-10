pub mod health;

use axum::{routing::get, Extension, Router};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::graphql::AppSchema;

pub fn router(schema: AppSchema) -> Router {
    Router::new()
        .nest("/api/v1", health::router())
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}

async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
