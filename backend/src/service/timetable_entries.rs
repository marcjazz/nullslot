use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::TimetableEntry;
use crate::repository::TimetableEntryRepository;
use crate::ws::{Broadcaster, WebSocketMessage};

pub struct TimetableEntryService {
    repo: TimetableEntryRepository,
    broadcaster: Arc<Broadcaster>,
}

impl TimetableEntryService {
    pub fn new(repo: TimetableEntryRepository, broadcaster: Arc<Broadcaster>) -> Self {
        Self { repo, broadcaster }
    }

    pub async fn create_timetable_entry(
        &self,
        course_id: Uuid,
        room_id: Uuid,
        time_slot_id: Uuid,
        teacher_id: Uuid,
    ) -> AppResult<TimetableEntry> {
        let entry = TimetableEntry {
            id: Uuid::new_v4(),
            course_id,
            room_id,
            time_slot_id,
            teacher_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = self.repo.create(entry).await?;

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "TIMETABLE_CREATED".to_string(),
            payload: json!({ "id": result.id }),
        });

        Ok(result)
    }

    pub async fn get_timetable_entry(&self, id: Uuid) -> AppResult<Option<TimetableEntry>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_timetable_entries(&self) -> AppResult<Vec<TimetableEntry>> {
        self.repo.find_all().await
    }

    pub async fn update_timetable_entry(
        &self,
        id: Uuid,
        course_id: Option<Uuid>,
        room_id: Option<Uuid>,
        time_slot_id: Option<Uuid>,
        teacher_id: Option<Uuid>,
    ) -> AppResult<TimetableEntry> {
        let mut entry = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(c) = course_id {
            entry.course_id = c;
        }
        if let Some(r) = room_id {
            entry.room_id = r;
        }
        if let Some(ts) = time_slot_id {
            entry.time_slot_id = ts;
        }
        if let Some(t) = teacher_id {
            entry.teacher_id = t;
        }
        
        entry.updated_at = Utc::now();
        let result = self.repo.update(entry).await?;

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "TIMETABLE_UPDATED".to_string(),
            payload: json!({ "id": result.id }),
        });

        Ok(result)
    }

    pub async fn delete_timetable_entry(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await?;

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "TIMETABLE_DELETED".to_string(),
            payload: json!({ "id": id }),
        });

        Ok(())
    }
}
