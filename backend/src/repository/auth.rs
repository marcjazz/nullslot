use sqlx::PgPool;
use crate::error::AppResult;
use crate::models::magic_link::MagicLink;

#[derive(Clone)]
pub struct AuthRepository {
    db_pool: PgPool,
}

impl AuthRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create_magic_link(&self, magic_link: MagicLink) -> AppResult<MagicLink> {
        let row = sqlx::query_as!(
            MagicLink,
            r#"
            INSERT INTO magic_links (token_hash, user_id, expires_at, used, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING token_hash, user_id, expires_at, used, created_at
            "#,
            magic_link.token_hash,
            magic_link.user_id,
            magic_link.expires_at,
            magic_link.used,
            magic_link.created_at
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(row)
    }

    pub async fn get_magic_link_by_hash(&self, token_hash: &str) -> AppResult<Option<MagicLink>> {
        let row = sqlx::query_as!(
            MagicLink,
            r#"
            SELECT token_hash, user_id, expires_at, used, created_at
            FROM magic_links
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(row)
    }

    pub async fn mark_magic_link_as_used(&self, token_hash: &str) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE magic_links
            SET used = TRUE
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
