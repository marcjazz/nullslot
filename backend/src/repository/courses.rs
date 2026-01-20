use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Course;
use crate::error::AppResult;

#[derive(Clone)]
pub struct CourseRepository {
    pool: PgPool,
}

impl CourseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, course: Course) -> AppResult<Course> {
        sqlx::query!(
            r#"
            INSERT INTO courses (id, workspace_id, code, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            course.id,
            course.workspace_id,
            course.code,
            course.name,
            course.description,
            course.created_at,
            course.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(course)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Course>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, workspace_id, code, name, description, created_at, updated_at
            FROM courses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Course>> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT id, workspace_id, code, name, description, created_at, updated_at
            FROM courses
            ORDER BY code ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(courses)
    }

    pub async fn update(&self, course: Course) -> AppResult<Course> {
        sqlx::query!(
            r#"
            UPDATE courses
            SET code = $2, name = $3, description = $4, updated_at = NOW()
            WHERE id = $1
            "#,
            course.id,
            course.code,
            course.name,
            course.description
        )
        .execute(&self.pool)
        .await?;

        Ok(course)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM courses
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
