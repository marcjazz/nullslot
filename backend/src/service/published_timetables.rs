use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::repository::PublishedTimetableRepository;
use crate::error::{AppResult, AppError};
use crate::models::published_timetables::PublishedTimetable;
use crate::models::draft_timetables::DraftTimetableStatus;
use crate::models::conflicts::ConflictStatus;
use crate::service::{DraftTimetableService, ConflictService};

pub struct PublishedTimetableService {
    repo: PublishedTimetableRepository,
    draft_timetable_service: Arc<DraftTimetableService>,
    conflict_service: Arc<ConflictService>,
}

impl PublishedTimetableService {
    pub fn new(
        repo: PublishedTimetableRepository,
        draft_timetable_service: Arc<DraftTimetableService>,
        conflict_service: Arc<ConflictService>,
    ) -> Self {
        Self {
            repo,
            draft_timetable_service,
            conflict_service,
        }
    }

    pub async fn publish_timetable(&self, workspace_id: Uuid, draft_timetable_id: Uuid) -> AppResult<PublishedTimetable> {
        // 1. Fetch the draft timetable
        let _draft = self.draft_timetable_service.get_draft(workspace_id, draft_timetable_id).await?
            .ok_or(AppError::NotFound)?;

        // 2. Fetch any conflicts and ensure they are all resolved
        let conflicts = self.conflict_service.get_conflicts_for_draft(draft_timetable_id).await?; // Assuming get_conflicts_for_draft doesn't need workspace_id
        if conflicts.iter().any(|c| c.status != ConflictStatus::Resolved) {
            return Err(AppError::UnprocessableEntity("Cannot publish timetable with unresolved conflicts".to_string()));
        }

        // 3. Create a new PublishedTimetable record
        let now = Utc::now();
        let published = PublishedTimetable {
            id: Uuid::new_v4(),
            workspace_id,
            draft_timetable_id,
            published_at: now,
            // For now, using current date as placeholder for valid_from/to if not available in draft
            valid_from: now.date_naive(),
            valid_to: now.date_naive() + chrono::Duration::days(180), // Default to 6 months
            created_at: now,
            updated_at: now,
        };

        let published = self.repo.create(published).await?;

        // 4. Update the status of the original DraftTimetable
        self.draft_timetable_service
            .update_draft_status(workspace_id, draft_timetable_id, DraftTimetableStatus::Published)
            .await?;

        Ok(published)
    }

    pub async fn get_published_timetable(&self, id: Uuid) -> AppResult<Option<PublishedTimetable>> {
        self.repo.get_by_id(id).await
    }

    pub async fn get_latest_published_timetable(&self) -> AppResult<Option<PublishedTimetable>> {
        self.repo.get_latest().await
    }
}
