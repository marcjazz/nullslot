use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_graphql::{Enum, SimpleObject};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "workspace_role")]
pub enum WorkspaceRole {
    Owner,
    Editor,
    Viewer,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, sqlx::FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub domain_restriction: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, sqlx::FromRow)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: WorkspaceRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, sqlx::FromRow)]
pub struct WorkspaceInvite {
    pub token_hash: String,
    pub workspace_id: Uuid,
    pub email: String,
    pub role: WorkspaceRole,
    pub expires_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}
