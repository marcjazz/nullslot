use crate::repository::DraftEntryRepository;
use crate::error::AppResult;
use crate::models::draft_entries::DraftEntry;
use crate::graphql::types::DraftEntryInput;
use uuid::Uuid;
use chrono::Utc;

pub struct DraftEntryService {
    repo: DraftEntryRepository,
}

impl DraftEntryService {
    pub fn new(repo: DraftEntryRepository) -> Self {
        Self { repo }
    }

    pub async fn add_entries_to_draft(&self, draft_timetable_id: Uuid, entries: Vec<DraftEntryInput>) -> AppResult<Vec<DraftEntry>> {
        let now = Utc::now();
        let draft_entries = entries.into_iter().map(|input| DraftEntry {
            id: Uuid::new_v4(),
            draft_timetable_id,
            course_id: input.course_id,
            teacher_id: input.teacher_id,
            room_id: input.room_id,
            time_slot_id: input.time_slot_id,
            created_at: now,
            updated_at: now,
        }).collect();

        self.repo.create_many(draft_entries).await
    }

    pub async fn get_entries_for_draft(&self, draft_timetable_id: Uuid) -> AppResult<Vec<DraftEntry>> {
        self.repo.get_by_draft_id(draft_timetable_id).await
    }
}
