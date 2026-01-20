use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "draft_timetable_status", rename_all = "lowercase")]
pub enum DraftTimetableStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct DraftTimetable {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub term: String,
    pub year: i32,
    pub status: DraftTimetableStatus,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
