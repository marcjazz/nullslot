use sqlx::PgPool;
use uuid::Uuid;
use crate::models::TimetableEntry;
use crate::error::AppResult;

#[derive(Clone)]
pub struct TimetableEntryRepository {
    pool: PgPool,
}

impl TimetableEntryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, entry: TimetableEntry) -> AppResult<TimetableEntry> {
        sqlx::query!(
            r#"
            INSERT INTO timetable_entries (id, course_id, room_id, time_slot_id, teacher_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            entry.id,
            entry.course_id,
            entry.room_id,
            entry.time_slot_id,
            entry.teacher_id,
            entry.created_at,
            entry.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(entry)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TimetableEntry>> {
        let entry = sqlx::query_as!(
            TimetableEntry,
            r#"
            SELECT id, course_id, room_id, time_slot_id, teacher_id, created_at, updated_at
            FROM timetable_entries
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(entry)
    }

    pub async fn find_all(&self) -> AppResult<Vec<TimetableEntry>> {
        let entries = sqlx::query_as!(
            TimetableEntry,
            r#"
            SELECT id, course_id, room_id, time_slot_id, teacher_id, created_at, updated_at
            FROM timetable_entries
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    pub async fn update(&self, entry: TimetableEntry) -> AppResult<TimetableEntry> {
        sqlx::query!(
            r#"
            UPDATE timetable_entries
            SET course_id = $2, room_id = $3, time_slot_id = $4, teacher_id = $5, updated_at = NOW()
            WHERE id = $1
            "#,
            entry.id,
            entry.course_id,
            entry.room_id,
            entry.time_slot_id,
            entry.teacher_id
        )
        .execute(&self.pool)
        .await?;

        Ok(entry)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM timetable_entries
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
