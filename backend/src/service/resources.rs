use uuid::Uuid;
use chrono::Utc;
use crate::models::Resource;
use crate::repository::ResourceRepository;
use crate::error::AppResult;

pub struct ResourceService {
    repo: ResourceRepository,
}

impl ResourceService {
    pub fn new(repo: ResourceRepository) -> Self {
        Self { repo }
    }

    pub async fn create_resource(
        &self,
        owner_id: Uuid,
        name: String,
        description: Option<String>,
        metadata: serde_json::Value,
    ) -> AppResult<Resource> {
        let resource = Resource {
            id: Uuid::new_v4(),
            owner_id,
            name,
            description,
            metadata,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(resource).await
    }

    pub async fn get_resource(&self, id: Uuid) -> AppResult<Option<Resource>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_resources(&self) -> AppResult<Vec<Resource>> {
        self.repo.find_all().await
    }

    pub async fn get_resources_by_owner(&self, owner_id: Uuid) -> AppResult<Vec<Resource>> {
        self.repo.find_by_owner_id(owner_id).await
    }

    pub async fn update_resource(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> AppResult<Resource> {
        let mut resource = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(n) = name {
            resource.name = n;
        }
        if let Some(d) = description {
            resource.description = Some(d);
        }
        if let Some(m) = metadata {
            resource.metadata = m;
        }
        
        resource.updated_at = Utc::now();
        self.repo.update(resource).await
    }

    pub async fn delete_resource(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await
    }
}
