use async_graphql::InputObject;
use uuid::Uuid;
use chrono::NaiveTime;
pub use crate::models::availability::Availability;
pub use crate::models::conflicts::{Conflict, ConflictStatus, ConflictSeverity};
pub use crate::models::draft_timetables::{DraftTimetable, DraftTimetableStatus};
pub use crate::models::published_timetables::PublishedTimetable;
pub use crate::models::draft_entries::DraftEntry;

#[derive(InputObject, Clone)]
pub struct AvailabilityInput {
    pub teacher_id: Uuid,
    pub day_of_week: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_preferred: bool,
}

#[derive(InputObject, Clone)]
pub struct DraftTimetableInput {
    pub name: String,
    pub term: String,
    pub year: i32,
    pub is_active: bool,
    pub entries: Vec<DraftEntryInput>,
}

#[derive(InputObject, Clone)]
pub struct DraftEntryInput {
    pub draft_timetable_id: Uuid,
    pub course_id: Uuid,
    pub teacher_id: Uuid,
    pub room_id: Uuid,
    pub time_slot_id: Uuid,
}
