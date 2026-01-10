use crate::models::{Course, Room, TimeSlot, User, TimetableEntry};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct TimetableSnapshot {
    pub courses: Vec<Course>,
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub teachers: Vec<User>,
    pub timetable_entries: Vec<TimetableEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, async_graphql::SimpleObject)]
pub struct Snapshot {
    pub id: Uuid,
    pub data: serde_json::Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}
