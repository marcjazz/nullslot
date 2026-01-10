use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // seconds
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct Resource {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
