use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppResult;
use crate::models::published_timetables::PublishedTimetable;

#[derive(Clone)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, timetable: PublishedTimetable) -> AppResult<PublishedTimetable> {
        let record = sqlx::query_as::<_, PublishedTimetable>(
            r#"
            INSERT INTO published_timetables (
                id, draft_timetable_id, published_at, valid_from, valid_to, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(timetable.id)
        .bind(timetable.draft_timetable_id)
        .bind(timetable.published_at)
        .bind(timetable.valid_from)
        .bind(timetable.valid_to)
        .bind(timetable.created_at)
        .bind(timetable.updated_at)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(record)
    }

    pub async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PublishedTimetable>> {
        let record = sqlx::query_as::<_, PublishedTimetable>(
            r#"
            SELECT * FROM published_timetables WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(record)
    }

    pub async fn get_latest(&self) -> AppResult<Option<PublishedTimetable>> {
        let record = sqlx::query_as::<_, PublishedTimetable>(
            r#"
            SELECT * FROM published_timetables
            ORDER BY published_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(record)
    }
}
