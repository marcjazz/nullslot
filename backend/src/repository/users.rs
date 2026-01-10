use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{User, UserRole};
use crate::error::AppResult;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: User) -> AppResult<User> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, hashed_password, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user.id,
            user.username,
            user.email,
            user.hashed_password,
            user.role as UserRole,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, hashed_password, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, hashed_password, role as "role: UserRole", created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, hashed_password, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, user: User) -> AppResult<User> {
        sqlx::query!(
            r#"
            UPDATE users
            SET username = $2, email = $3, hashed_password = $4, role = $5, updated_at = NOW()
            WHERE id = $1
            "#,
            user.id,
            user.username,
            user.email,
            user.hashed_password,
            user.role as UserRole
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
