pub mod health;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::get, Extension, Router};
use std::sync::Arc;

use crate::graphql::AppSchema;
use crate::ws::Broadcaster;

pub fn router(schema: AppSchema, broadcaster: Arc<Broadcaster>) -> Router {
    Router::new()
        .nest("/api/v1", health::router::<Arc<Broadcaster>>())
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/ws", get(crate::ws::ws_handler))
        .layer(Extension(schema))
        .with_state(broadcaster)
}

async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // For now, we mock an unauthenticated user.
    // In a real app, this would be extracted from a JWT token in the header.
    let user: Option<crate::models::User> = None;
    let request = req.into_inner().data(user);
    schema.execute(request).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
