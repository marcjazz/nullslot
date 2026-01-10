use uuid::Uuid;
use chrono::Utc;
use crate::models::Room;
use crate::repository::RoomRepository;
use crate::error::AppResult;

pub struct RoomService {
    repo: RoomRepository,
}

impl RoomService {
    pub fn new(repo: RoomRepository) -> Self {
        Self { repo }
    }

    pub async fn create_room(
        &self,
        name: String,
        capacity: i32,
    ) -> AppResult<Room> {
        let room = Room {
            id: Uuid::new_v4(),
            name,
            capacity,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(room).await
    }

    pub async fn get_room(&self, id: Uuid) -> AppResult<Option<Room>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_rooms(&self) -> AppResult<Vec<Room>> {
        self.repo.find_all().await
    }

    pub async fn update_room(
        &self,
        id: Uuid,
        name: Option<String>,
        capacity: Option<i32>,
    ) -> AppResult<Room> {
        let mut room = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(n) = name {
            room.name = n;
        }
        if let Some(c) = capacity {
            room.capacity = c;
        }
        
        room.updated_at = Utc::now();
        self.repo.update(room).await
    }

    pub async fn delete_room(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await
    }
}
