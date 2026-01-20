use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Room;
use crate::error::AppResult;

#[derive(Clone)]
pub struct RoomRepository {
    pool: PgPool,
}

impl RoomRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, room: Room) -> AppResult<Room> {
        sqlx::query!(
            r#"
            INSERT INTO rooms (id, workspace_id, name, capacity, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            room.id,
            room.workspace_id,
            room.name,
            room.capacity,
            room.created_at,
            room.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(room)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Room>> {
        let room = sqlx::query_as!(
            Room,
            r#"
            SELECT id, workspace_id, name, capacity, created_at, updated_at
            FROM rooms
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(room)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
            SELECT id, workspace_id, name, capacity, created_at, updated_at
            FROM rooms
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }

    pub async fn update(&self, room: Room) -> AppResult<Room> {
        sqlx::query!(
            r#"
            UPDATE rooms
            SET name = $2, capacity = $3, updated_at = NOW()
            WHERE id = $1
            "#,
            room.id,
            room.name,
            room.capacity
        )
        .execute(&self.pool)
        .await?;

        Ok(room)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM rooms
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
