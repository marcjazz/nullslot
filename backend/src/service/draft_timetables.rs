use crate::error::AppResult;
use crate::models::draft_timetables::{DraftTimetable, DraftTimetableStatus};
use crate::repository::DraftTimetableRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct DraftTimetableService {
    repo: DraftTimetableRepository,
}

impl DraftTimetableService {
    pub fn new(repo: DraftTimetableRepository) -> Self {
        Self { repo }
    }

    pub async fn create_draft(
        &self,
        workspace_id: Uuid,
        name: String,
        term: String,
        year: i32,
    ) -> AppResult<DraftTimetable> {
        let draft = DraftTimetable {
            id: Uuid::new_v4(),
            workspace_id,
            name,
            term,
            year,
            status: DraftTimetableStatus::Draft,
            is_active: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(draft).await
    }

    pub async fn get_draft(
        &self,
        workspace_id: Uuid,
        id: Uuid,
    ) -> AppResult<Option<DraftTimetable>> {
        self.repo.get_by_id(workspace_id, id).await
    }

    pub async fn update_draft_status(
        &self,
        workspace_id: Uuid,
        id: Uuid,
        status: DraftTimetableStatus,
    ) -> AppResult<DraftTimetable> {
        self.repo.update_status(workspace_id, id, status).await
    }
}
