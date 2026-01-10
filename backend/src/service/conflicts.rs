use crate::repository::{ConflictRepository, TimeSlotRepository};
use crate::error::AppResult;
use crate::models::conflicts::{Conflict, ConflictStatus};
use crate::service::{DraftEntryService, AvailabilityService};
use uuid::Uuid;
use std::sync::Arc;
use chrono::Utc;

pub struct ConflictService {
    repo: ConflictRepository,
    draft_entry_service: Arc<DraftEntryService>,
    availability_service: Arc<AvailabilityService>,
    time_slot_repo: TimeSlotRepository,
}

impl ConflictService {
    pub fn new(
        repo: ConflictRepository,
        draft_entry_service: Arc<DraftEntryService>,
        availability_service: Arc<AvailabilityService>,
        time_slot_repo: TimeSlotRepository,
    ) -> Self {
        Self {
            repo,
            draft_entry_service,
            availability_service,
            time_slot_repo,
        }
    }

    pub async fn get_conflicts(&self, draft_timetable_id: Uuid) -> AppResult<Vec<Conflict>> {
        let entries = self.draft_entry_service.get_entries_for_draft(draft_timetable_id).await?;
        let mut conflicts = Vec::new();

        // 1. Teacher Double-Booking
        for (i, entry1) in entries.iter().enumerate() {
            for entry2 in entries.iter().skip(i + 1) {
                if entry1.teacher_id == entry2.teacher_id && entry1.time_slot_id == entry2.time_slot_id {
                    let description = format!(
                        "Teacher is double-booked for courses {} and {} at the same time",
                        entry1.course_id, entry2.course_id
                    );
                    let conflict = Conflict {
                        id: Uuid::new_v4(),
                        draft_timetable_id,
                        description,
                        teacher_id: Some(entry1.teacher_id),
                        room_id: None,
                        time_slot_id: Some(entry1.time_slot_id),
                        status: ConflictStatus::Open,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    conflicts.push(self.repo.create(conflict).await?);
                }
            }
        }

        // 2. Room Double-Booking
        for (i, entry1) in entries.iter().enumerate() {
            for entry2 in entries.iter().skip(i + 1) {
                if entry1.room_id == entry2.room_id && entry1.time_slot_id == entry2.time_slot_id {
                    let description = format!(
                        "Room is double-booked for courses {} and {} at the same time",
                        entry1.course_id, entry2.course_id
                    );
                    let conflict = Conflict {
                        id: Uuid::new_v4(),
                        draft_timetable_id,
                        description,
                        teacher_id: None,
                        room_id: Some(entry1.room_id),
                        time_slot_id: Some(entry1.time_slot_id),
                        status: ConflictStatus::Open,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    conflicts.push(self.repo.create(conflict).await?);
                }
            }
        }

        // 3. Availability Check (Optional Bonus)
        for entry in &entries {
            if let Some(time_slot) = self.time_slot_repo.find_by_id(entry.time_slot_id).await? {
                // Mock a date based on day_of_week for the availability service
                // Day 0 is Sunday in chrono weekday() but we need to check what AvailabilityRepository expects.
                // AvailabilityRepository::get_by_teacher_and_date calculates day_of_week = date.weekday().num_days_from_sunday() as i32;
                // So 0 is Sunday, 1 is Monday, ..., 6 is Saturday.
                
                // We can use a reference date that we know is a Sunday, e.g., 2026-01-04 (which is a Sunday)
                let base_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();
                let date = base_date + chrono::Duration::days(time_slot.day_of_week as i64);
                
                let availabilities = self.availability_service.get_availability(entry.teacher_id, date).await?;
                
                let is_available = availabilities.iter().any(|a| {
                    a.start_time <= time_slot.start_time && a.end_time >= time_slot.end_time
                });

                if !is_available {
                    let description = format!(
                        "Teacher is not available during time slot {}",
                        entry.time_slot_id
                    );
                    let conflict = Conflict {
                        id: Uuid::new_v4(),
                        draft_timetable_id,
                        description,
                        teacher_id: Some(entry.teacher_id),
                        room_id: None,
                        time_slot_id: Some(entry.time_slot_id),
                        status: ConflictStatus::Open,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    conflicts.push(self.repo.create(conflict).await?);
                }
            }
        }

        Ok(conflicts)
    }

    pub async fn resolve_conflict(&self, conflict_id: Uuid, status: ConflictStatus) -> AppResult<Conflict> {
        self.repo.update_status(conflict_id, status).await
    }

    pub async fn get_conflicts_for_draft(&self, draft_id: Uuid) -> AppResult<Vec<Conflict>> {
        self.repo.get_by_draft_timetable(draft_id).await
    }
}
