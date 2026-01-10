use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppResult;
use crate::models::draft_entries::DraftEntry;

#[derive(Clone)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create_many(&self, entries: Vec<DraftEntry>) -> AppResult<Vec<DraftEntry>> {
        let mut tx = self.db_pool.begin().await?;

        let mut created_entries = Vec::with_capacity(entries.len());

        for entry in entries {
            let created = sqlx::query_as!(
                DraftEntry,
                r#"
                INSERT INTO draft_entries (id, draft_timetable_id, course_id, teacher_id, room_id, time_slot_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, draft_timetable_id, course_id, teacher_id, room_id, time_slot_id, created_at, updated_at
                "#,
                entry.id,
                entry.draft_timetable_id,
                entry.course_id,
                entry.teacher_id,
                entry.room_id,
                entry.time_slot_id,
                entry.created_at,
                entry.updated_at
            )
            .fetch_one(&mut *tx)
            .await?;
            created_entries.push(created);
        }

        tx.commit().await?;

        Ok(created_entries)
    }

    pub async fn get_by_draft_id(&self, draft_id: Uuid) -> AppResult<Vec<DraftEntry>> {
        let entries = sqlx::query_as!(
            DraftEntry,
            r#"
            SELECT id, draft_timetable_id, course_id, teacher_id, room_id, time_slot_id, created_at, updated_at
            FROM draft_entries
            WHERE draft_timetable_id = $1
            "#,
            draft_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(entries)
    }
}
