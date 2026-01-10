use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Datelike};
use crate::error::AppResult;
use crate::models::availability::Availability;

#[derive(Clone)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, availability: Availability) -> AppResult<Availability> {
        let row = sqlx::query_as!(
            Availability,
            r#"
            INSERT INTO availability (id, teacher_id, day_of_week, start_time, end_time, is_preferred, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, teacher_id, day_of_week, start_time, end_time, is_preferred, created_at, updated_at
            "#,
            availability.id,
            availability.teacher_id,
            availability.day_of_week,
            availability.start_time,
            availability.end_time,
            availability.is_preferred,
            availability.created_at,
            availability.updated_at
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_teacher_and_date(&self, teacher_id: Uuid, date: NaiveDate) -> AppResult<Vec<Availability>> {
        let day_of_week = date.weekday().num_days_from_sunday() as i32;

        let rows = sqlx::query_as!(
            Availability,
            r#"
            SELECT id, teacher_id, day_of_week, start_time, end_time, is_preferred, created_at, updated_at
            FROM availability
            WHERE teacher_id = $1 AND day_of_week = $2
            "#,
            teacher_id,
            day_of_week
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(rows)
    }
}
