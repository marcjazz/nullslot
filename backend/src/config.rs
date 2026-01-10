use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub oidc_issuer_url: String,
    pub oidc_redirect_uri: String,
    pub oidc_frontend_redirect_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            oidc_client_id: env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID must be set"),
            oidc_client_secret: env::var("OIDC_CLIENT_SECRET").expect("OIDC_CLIENT_SECRET must be set"),
            oidc_issuer_url: env::var("OIDC_ISSUER_URL").expect("OIDC_ISSUER_URL must be set"),
            oidc_redirect_uri: env::var("OIDC_REDIRECT_URI").expect("OIDC_REDIRECT_URI must be set"),
            oidc_frontend_redirect_url: env::var("OIDC_FRONTEND_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:5173/oidc-callback".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string()),
        }
    }
}
