use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppResult;
use crate::models::conflicts::{Conflict, ConflictStatus};

#[derive(Clone)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, conflict: Conflict) -> AppResult<Conflict> {
        let result = sqlx::query_as!(
            Conflict,
            r#"
            INSERT INTO conflicts (id, draft_timetable_id, description, teacher_id, room_id, time_slot_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, draft_timetable_id, description, teacher_id, room_id, time_slot_id, status as "status: ConflictStatus", created_at, updated_at
            "#,
            conflict.id,
            conflict.draft_timetable_id,
            conflict.description,
            conflict.teacher_id,
            conflict.room_id,
            conflict.time_slot_id,
            conflict.status as ConflictStatus,
            conflict.created_at,
            conflict.updated_at
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_draft_timetable(&self, draft_id: Uuid) -> AppResult<Vec<Conflict>> {
        let conflicts = sqlx::query_as!(
            Conflict,
            r#"
            SELECT id, draft_timetable_id, description, teacher_id, room_id, time_slot_id, status as "status: ConflictStatus", created_at, updated_at
            FROM conflicts
            WHERE draft_timetable_id = $1
            "#,
            draft_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(conflicts)
    }

    pub async fn update_status(&self, id: Uuid, status: ConflictStatus) -> AppResult<Conflict> {
        let result = sqlx::query_as!(
            Conflict,
            r#"
            UPDATE conflicts
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, draft_timetable_id, description, teacher_id, room_id, time_slot_id, status as "status: ConflictStatus", created_at, updated_at
            "#,
            id,
            status as ConflictStatus
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }
}
