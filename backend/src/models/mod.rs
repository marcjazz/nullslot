pub mod availability;
pub mod conflicts;
pub mod draft_entries;
pub mod draft_timetables;
pub mod published_timetables;
pub mod snapshot;

use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime};

pub use availability::Availability;
pub use conflicts::{Conflict, ConflictSeverity, ConflictStatus};
pub use draft_entries::DraftEntry;
pub use draft_timetables::{DraftTimetable, DraftTimetableStatus};
pub use published_timetables::PublishedTimetable;

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

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct Course {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct TimeSlot {
    pub id: Uuid,
    pub day_of_week: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct TimetableEntry {
    pub id: Uuid,
    pub course_id: Uuid,
    pub room_id: Uuid,
    pub time_slot_id: Uuid,
    pub teacher_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Admin,
    Teacher,
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "substitution_status")]
pub enum SubstitutionStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct Substitution {
    pub id: Uuid,
    pub timetable_entry_id: Uuid,
    pub substituting_teacher_id: Option<Uuid>,
    pub status: SubstitutionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
