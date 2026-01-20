use crate::repository::AvailabilityRepository;
use crate::error::AppResult;
use crate::graphql::types::AvailabilityInput;
use crate::models::availability::Availability;
use uuid::Uuid;
use chrono::Utc;

pub struct AvailabilityService {
    repo: AvailabilityRepository,
}

impl AvailabilityService {
    pub fn new(repo: AvailabilityRepository) -> Self {
        Self { repo }
    }

    pub async fn submit_availability(&self, workspace_id: Uuid, input: AvailabilityInput) -> AppResult<Availability> {
        let availability = Availability {
            id: Uuid::new_v4(),
            workspace_id,
            teacher_id: input.teacher_id,
            day_of_week: input.day_of_week,
            start_time: input.start_time,
            end_time: input.end_time,
            is_preferred: input.is_preferred,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(availability).await
    }

    pub async fn get_availability(&self, teacher_id: Uuid, date: chrono::NaiveDate) -> AppResult<Vec<Availability>> {
        self.repo.get_by_teacher_and_date(teacher_id, date).await
    }
}
