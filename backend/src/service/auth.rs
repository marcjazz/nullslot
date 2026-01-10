use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::magic_link::MagicLink;
use crate::models::{User, UserRole};
use crate::oidc::OidcClient;
use crate::repository::{AuthRepository, UserRepository};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub exp: i64,
}

use std::sync::Arc;

pub struct AuthService {
    auth_repo: AuthRepository,
    user_repo: UserRepository,
    config: Arc<Config>,
    oidc_client: Arc<OidcClient>,
}

impl AuthService {
    pub fn new(
        auth_repo: AuthRepository,
        user_repo: UserRepository,
        config: Arc<Config>,
        oidc_client: Arc<OidcClient>,
    ) -> Self {
        Self {
            auth_repo,
            user_repo,
            config,
            oidc_client,
        }
    }

    pub async fn request_magic_link(&self, email: &str) -> AppResult<String> {
        // Find the user in the database
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or(AppError::NotFound)?;

        // Generate a secure, random token
        let token: String = rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // Hash the token using Argon2. Do not store the raw token.
        // We use a deterministic approach here because we need to look up by hash.
        // In a real application, you might use a unique ID + raw token, or a faster deterministic hash for lookup.
        // For this task, we'll use a fixed salt to allow deterministic Argon2 hashing.
        let salt = SaltString::new("static_salt_for_lookup").unwrap();
        let argon2 = Argon2::default();
        let token_hash = argon2
            .hash_password(token.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("Hashing error: {}", e)))?
            .to_string();

        // Create a MagicLink object
        let magic_link = MagicLink {
            token_hash: token_hash.clone(),
            user_id: user.id,
            expires_at: Utc::now() + Duration::minutes(15),
            used: false,
            created_at: Utc::now(),
        };

        // Save the MagicLink to the database
        self.auth_repo.create_magic_link(magic_link).await?;

        // Simulate sending the email: log the raw token to the console
        tracing::info!("MAGIC LINK TOKEN: {}", token);
        println!("MAGIC LINK TOKEN: {}", token);

        Ok("Magic link sent to your email".to_string())
    }

    pub async fn login_with_magic_link(&self, token: &str) -> AppResult<(String, User)> {
        // Hash the incoming token so you can compare it with the stored hash
        let salt = SaltString::new("static_salt_for_lookup").unwrap();
        let argon2 = Argon2::default();
        let token_hash = argon2
            .hash_password(token.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("Hashing error: {}", e)))?
            .to_string();

        // Fetch the MagicLink from the database using the hashed token
        let magic_link = self
            .auth_repo
            .get_magic_link_by_hash(&token_hash)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Validate the token
        if magic_link.used {
            return Err(AppError::BadRequest("Magic link has already been used".to_string()));
        }

        if magic_link.expires_at < Utc::now() {
            return Err(AppError::BadRequest("Magic link has expired".to_string()));
        }

        // Mark it as used in the database
        self.auth_repo.mark_magic_link_as_used(&token_hash).await?;

        // Find the user to get their role
        let user = self
            .user_repo
            .find_by_id(magic_link.user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Generate a JWT for the user
        let jwt = self.create_jwt(user.id, user.role)?;

        Ok((jwt, user))
    }

    fn create_jwt(&self, user_id: Uuid, role: UserRole) -> AppResult<String> {
        let secret = &self.config.jwt_secret;
        let expiration = Utc::now() + Duration::hours(24);

        let claims = Claims {
            sub: user_id,
            role,
            exp: expiration.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("JWT generation error: {}", e)))
    }

    pub async fn login_with_oidc_code(&self, code: &str) -> AppResult<(String, User)> {
        use openidconnect::{AuthorizationCode, OAuth2TokenResponse, PkceCodeVerifier, TokenResponse};

        // 1. Exchange the code for an ID token
        // Note: Using a placeholder PKCE verifier as per instructions
        let pkce_verifier = PkceCodeVerifier::new("placeholder_pkce_verifier".to_string());

        let token_response = self
            .oidc_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(openidconnect::reqwest::async_http_client)
            .await
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("Failed to exchange OIDC code: {}", e)))?;

        // 2. Verify the ID token
        let id_token = token_response
            .id_token()
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Server did not return an ID token")))?;

        // Note: For verification, we ideally need the nonce used during the auth URL generation.
        // Since we are not using sessions yet, we use a placeholder or skip strict nonce verification if possible.
        // The openidconnect crate requires a nonce for verification if it was provided in the auth request.
        // For now, we'll try to verify it.
        let claims = id_token
            .claims(&self.oidc_client.id_token_verifier(), &openidconnect::Nonce::new("placeholder_nonce".to_string()))
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("Failed to verify ID token: {}", e)))?;

        // 3. Extract the user's email
        let email = claims
            .email()
            .ok_or_else(|| AppError::BadRequest("OIDC ID token does not contain an email claim".to_string()))?
            .as_str();

        // 4. Find or create the user
        let user = match self.user_repo.find_by_email(email).await? {
            Some(user) => user,
            None => {
                let new_user = User {
                    id: Uuid::new_v4(),
                    username: email.split('@').next().unwrap_or("unknown").to_string(),
                    email: email.to_string(),
                    hashed_password: "".to_string(), // SSO users don't have a local password
                    role: UserRole::User,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                self.user_repo.create(new_user).await?
            }
        };

        // 5. Generate a local JWT
        let jwt = self.create_jwt(user.id, user.role)?;

        Ok((jwt, user))
    }
}
