use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct PublishedTimetable {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub draft_timetable_id: Uuid,
    pub published_at: DateTime<Utc>,
    pub valid_from: NaiveDate,
    pub valid_to: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
