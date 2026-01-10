use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Resource;
use crate::error::AppResult;

pub struct ResourceRepository {
    pool: PgPool,
}

impl ResourceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, resource: Resource) -> AppResult<Resource> {
        sqlx::query!(
            r#"
            INSERT INTO resources (id, owner_id, name, description, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            resource.id,
            resource.owner_id,
            resource.name,
            resource.description,
            resource.metadata,
            resource.created_at,
            resource.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            eprintln!("SQLx Error in ResourceRepository::create: {:?}", e);
            e
        })?;

        Ok(resource)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Resource>> {
        let resource = sqlx::query_as!(
            Resource,
            r#"
            SELECT id, owner_id, name, description, metadata as "metadata: serde_json::Value", created_at, updated_at
            FROM resources
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(resource)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Resource>> {
        let resources = sqlx::query_as!(
            Resource,
            r#"
            SELECT id, owner_id, name, description, metadata as "metadata: serde_json::Value", created_at, updated_at
            FROM resources
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(resources)
    }

    pub async fn find_by_owner_id(&self, owner_id: Uuid) -> AppResult<Vec<Resource>> {
        let resources = sqlx::query_as!(
            Resource,
            r#"
            SELECT id, owner_id, name, description, metadata as "metadata: serde_json::Value", created_at, updated_at
            FROM resources
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(resources)
    }

    pub async fn update(&self, resource: Resource) -> AppResult<Resource> {
        sqlx::query!(
            r#"
            UPDATE resources
            SET name = $2, description = $3, metadata = $4, updated_at = NOW()
            WHERE id = $1
            "#,
            resource.id,
            resource.name,
            resource.description,
            resource.metadata
        )
        .execute(&self.pool)
        .await?;

        Ok(resource)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM resources
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
