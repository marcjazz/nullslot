use uuid::Uuid;
use chrono::{Utc, NaiveTime};
use crate::models::TimeSlot;
use crate::repository::TimeSlotRepository;
use crate::error::AppResult;

pub struct TimeSlotService {
    repo: TimeSlotRepository,
}

impl TimeSlotService {
    pub fn new(repo: TimeSlotRepository) -> Self {
        Self { repo }
    }

    pub async fn create_time_slot(
        &self,
        day_of_week: i32,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> AppResult<TimeSlot> {
        let time_slot = TimeSlot {
            id: Uuid::new_v4(),
            day_of_week,
            start_time,
            end_time,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(time_slot).await
    }

    pub async fn get_time_slot(&self, id: Uuid) -> AppResult<Option<TimeSlot>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_time_slots(&self) -> AppResult<Vec<TimeSlot>> {
        self.repo.find_all().await
    }

    pub async fn update_time_slot(
        &self,
        id: Uuid,
        day_of_week: Option<i32>,
        start_time: Option<NaiveTime>,
        end_time: Option<NaiveTime>,
    ) -> AppResult<TimeSlot> {
        let mut time_slot = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(d) = day_of_week {
            time_slot.day_of_week = d;
        }
        if let Some(s) = start_time {
            time_slot.start_time = s;
        }
        if let Some(e) = end_time {
            time_slot.end_time = e;
        }
        
        time_slot.updated_at = Utc::now();
        self.repo.update(time_slot).await
    }

    pub async fn delete_time_slot(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await
    }
}
