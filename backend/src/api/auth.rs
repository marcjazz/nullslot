use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use openidconnect::{
    core::CoreAuthenticationFlow,
    CsrfToken, PkceCodeChallenge, Scope,
};
use crate::{AppState, graphql::AppSchema, service::auth::AuthService};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OidcCallbackParams {
    pub code: String,
    pub state: String,
}

pub async fn oidc_login_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Generate PKCE challenge
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate authorization URL
    let (auth_url, _csrf_token, _nonce) = state.oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            openidconnect::Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    // TODO: Store pkce_verifier, csrf_token, and nonce in a session or temporary store
    // For now, we just redirect.

    Redirect::to(auth_url.as_str())
}

pub async fn oidc_callback_handler(
    State(state): State<AppState>,
    Extension(schema): Extension<AppSchema>,
    Query(params): Query<OidcCallbackParams>,
) -> impl IntoResponse {
    let auth_service = schema
        .data::<AuthService>()
        .expect("AuthService not found in schema data");

    match auth_service.login_with_oidc_code(&params.code).await {
        Ok((jwt, _user)) => {
            let redirect_url = format!(
                "{}?token={}",
                state.config.oidc_frontend_redirect_url,
                jwt
            );
            Redirect::to(&redirect_url)
        }
        Err(e) => {
            tracing::error!("OIDC login failed: {:?}", e);
            // Redirect to login page with error
            let redirect_url = format!(
                "{}?error=oidc_login_failed",
                state.config.oidc_frontend_redirect_url
            );
            Redirect::to(&redirect_url)
        }
    }
}
