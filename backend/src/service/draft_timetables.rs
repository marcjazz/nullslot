use crate::repository::DraftTimetableRepository;
use crate::error::AppResult;
use crate::graphql::types::DraftTimetableInput;
use crate::models::draft_timetables::{DraftTimetable, DraftTimetableStatus};
use uuid::Uuid;
use chrono::Utc;

pub struct DraftTimetableService {
    repo: DraftTimetableRepository,
}

impl DraftTimetableService {
    pub fn new(repo: DraftTimetableRepository) -> Self {
        Self { repo }
    }

    pub async fn create_draft_timetable(&self, input: DraftTimetableInput) -> AppResult<DraftTimetable> {
        let draft = DraftTimetable {
            id: Uuid::new_v4(),
            name: input.name,
            term: input.term,
            year: input.year,
            status: DraftTimetableStatus::Draft,
            is_active: input.is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create(draft).await
    }

    pub async fn get_draft_timetable(&self, id: Uuid) -> AppResult<Option<DraftTimetable>> {
        self.repo.get_by_id(id).await
    }

    pub async fn update_draft_timetable_status(&self, id: Uuid, status: DraftTimetableStatus) -> AppResult<DraftTimetable> {
        self.repo.update_status(id, status).await
    }
}
