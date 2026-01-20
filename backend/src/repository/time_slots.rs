use sqlx::PgPool;
use uuid::Uuid;
use crate::models::TimeSlot;
use crate::error::AppResult;

#[derive(Clone)]
pub struct TimeSlotRepository {
    pool: PgPool,
}

impl TimeSlotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, time_slot: TimeSlot) -> AppResult<TimeSlot> {
        sqlx::query!(
            r#"
            INSERT INTO time_slots (id, workspace_id, day_of_week, start_time, end_time, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            time_slot.id,
            time_slot.workspace_id,
            time_slot.day_of_week,
            time_slot.start_time,
            time_slot.end_time,
            time_slot.created_at,
            time_slot.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(time_slot)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TimeSlot>> {
        let time_slot = sqlx::query_as!(
            TimeSlot,
            r#"
            SELECT id, workspace_id, day_of_week, start_time, end_time, created_at, updated_at
            FROM time_slots
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(time_slot)
    }

    pub async fn find_all(&self) -> AppResult<Vec<TimeSlot>> {
        let time_slots = sqlx::query_as!(
            TimeSlot,
            r#"
            SELECT id, workspace_id, day_of_week, start_time, end_time, created_at, updated_at
            FROM time_slots
            ORDER BY day_of_week ASC, start_time ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(time_slots)
    }

    pub async fn update(&self, time_slot: TimeSlot) -> AppResult<TimeSlot> {
        sqlx::query!(
            r#"
            UPDATE time_slots
            SET day_of_week = $2, start_time = $3, end_time = $4, updated_at = NOW()
            WHERE id = $1
            "#,
            time_slot.id,
            time_slot.day_of_week,
            time_slot.start_time,
            time_slot.end_time
        )
        .execute(&self.pool)
        .await?;

        Ok(time_slot)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM time_slots
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
