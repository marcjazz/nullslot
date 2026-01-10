use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "conflict_status")]
pub enum ConflictStatus {
    Open,
    Resolved,
    Ignored,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct Conflict {
    pub id: Uuid,
    pub draft_timetable_id: Uuid,
    pub description: String,
    pub teacher_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Option<Uuid>,
    pub status: ConflictStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
