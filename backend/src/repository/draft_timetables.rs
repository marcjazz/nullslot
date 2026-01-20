use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppResult;
use crate::models::draft_timetables::{DraftTimetable, DraftTimetableStatus};

#[derive(Clone)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, draft: DraftTimetable) -> AppResult<DraftTimetable> {
        let row = sqlx::query_as!(
            DraftTimetable,
            r#"
            INSERT INTO draft_timetables (id, workspace_id, name, term, year, status, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, workspace_id, name, term, year, status as "status: DraftTimetableStatus", is_active, created_at, updated_at
            "#,
            draft.id,
            draft.workspace_id,
            draft.name,
            draft.term,
            draft.year,
            draft.status as DraftTimetableStatus,
            draft.is_active,
            draft.created_at,
            draft.updated_at
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_id(&self, workspace_id: Uuid, id: Uuid) -> AppResult<Option<DraftTimetable>> {
        let row = sqlx::query_as!(
            DraftTimetable,
            r#"
            SELECT id, workspace_id, name, term, year, status as "status: DraftTimetableStatus", is_active, created_at, updated_at
            FROM draft_timetables
            WHERE id = $1 AND workspace_id = $2
            "#,
            id,
            workspace_id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(row)
    }

    pub async fn update_status(&self, workspace_id: Uuid, id: Uuid, status: DraftTimetableStatus) -> AppResult<DraftTimetable> {
        let row = sqlx::query_as!(
            DraftTimetable,
            r#"
            UPDATE draft_timetables
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND workspace_id = $3
            RETURNING id, workspace_id, name, term, year, status as "status: DraftTimetableStatus", is_active, created_at, updated_at
            "#,
            status as DraftTimetableStatus,
            id,
            workspace_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(row)
    }
}
