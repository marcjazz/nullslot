use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct MagicLink {
    pub token_hash: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}
