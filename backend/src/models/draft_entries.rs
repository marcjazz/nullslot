use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct DraftEntry {
    pub id: Uuid,
    pub draft_timetable_id: Uuid,
    pub course_id: Uuid,
    pub teacher_id: Uuid,
    pub room_id: Uuid,
    pub time_slot_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
