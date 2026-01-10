use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::header,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::error::AppError;
use crate::service::auth::Claims;

pub async fn auth(
    State(state): State<crate::AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(auth_header) if auth_header.starts_with("Bearer ") => {
            &auth_header[7..]
        }
        _ => return Err(AppError::Unauthorized),
    };

    let secret = &state.config.jwt_secret;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::error!("JWT validation error: {:?}", e);
        AppError::Unauthorized
    })?;

    // Insert claims into request extensions so they can be accessed by GraphQL resolvers
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
