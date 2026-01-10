use crate::models::{Course, Room, TimeSlot, User, TimetableEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, async_graphql::SimpleObject)]
pub struct TimetableSnapshot {
    pub courses: Vec<Course>,
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub teachers: Vec<User>,
    pub timetable_entries: Vec<TimetableEntry>,
}
