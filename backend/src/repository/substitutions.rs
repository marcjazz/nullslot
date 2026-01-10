use sqlx::PgPool;
use uuid::Uuid;
#[allow(unused_imports)]
use crate::models::{Substitution, SubstitutionStatus};
use crate::error::AppResult;

#[derive(Clone)]
pub struct SubstitutionRepository {
    pool: PgPool,
}

impl SubstitutionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, substitution: Substitution) -> AppResult<Substitution> {
        sqlx::query!(
            r#"
            INSERT INTO substitutions (id, timetable_entry_id, substituting_teacher_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            substitution.id,
            substitution.timetable_entry_id,
            substitution.substituting_teacher_id,
            substitution.status as SubstitutionStatus,
            substitution.created_at,
            substitution.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(substitution)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Substitution>> {
        let substitution = sqlx::query_as!(
            Substitution,
            r#"
            SELECT id, timetable_entry_id, substituting_teacher_id, status as "status: SubstitutionStatus", created_at, updated_at
            FROM substitutions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(substitution)
    }

    pub async fn update(&self, substitution: Substitution) -> AppResult<Substitution> {
        sqlx::query!(
            r#"
            UPDATE substitutions
            SET substituting_teacher_id = $2, status = $3, updated_at = NOW()
            WHERE id = $1
            "#,
            substitution.id,
            substitution.substituting_teacher_id,
            substitution.status as SubstitutionStatus
        )
        .execute(&self.pool)
        .await?;

        Ok(substitution)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Substitution>> {
        let substitutions = sqlx::query_as!(
            Substitution,
            r#"
            SELECT id, timetable_entry_id, substituting_teacher_id, status as "status: SubstitutionStatus", created_at, updated_at
            FROM substitutions
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(substitutions)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM substitutions
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
