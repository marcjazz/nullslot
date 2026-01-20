pub mod health;
pub mod auth;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderMap,
    routing::get,
    Extension, Router,
};
use uuid::Uuid;

use crate::graphql::{AppSchema, WorkspaceContext};
use crate::AppState;

pub fn router(
    schema: AppSchema,
    state: AppState,
) -> Router {
    Router::new()
        .nest("/api/v1", health::router::<AppState>())
        .route("/auth/oidc/login", get(auth::oidc_login_handler))
        .route("/auth/oidc/callback", get(auth::oidc_callback_handler))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/ws", get(crate::ws::ws_handler))
        .layer(Extension(schema))
        .with_state(state)
}

async fn graphql_handler(
    schema: Extension<AppSchema>,
    claims: Option<axum::extract::Extension<crate::service::auth::Claims>>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    if let Some(axum::extract::Extension(claims)) = claims {
        request = request.data(claims);
    }

    let workspace_id = headers
        .get("X-Workspace-ID")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    request = request.data(WorkspaceContext { workspace_id });

    schema.execute(request).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
