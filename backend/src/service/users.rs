use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, UserRole};
use crate::repository::UserRepository;
use crate::error::AppResult;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, username: String, email: String, password: String) -> AppResult<User> {
        let hashed_password = password; // In a real app, hash the password
        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            hashed_password,
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(user).await
    }

    pub async fn get_user(&self, id: Uuid) -> AppResult<Option<User>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_users(&self) -> AppResult<Vec<User>> {
        self.repo.find_all().await
    }

    pub async fn update_user(&self, id: Uuid, username: Option<String>, email: Option<String>) -> AppResult<User> {
        let mut user = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(u) = username {
            user.username = u;
        }
        if let Some(e) = email {
            user.email = e;
        }
        
        user.updated_at = Utc::now();
        self.repo.update(user).await
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await
    }
}
