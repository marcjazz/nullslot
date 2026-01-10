use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{Substitution, SubstitutionStatus};
use crate::repository::SubstitutionRepository;
use crate::service::NotificationService;
use crate::ws::{Broadcaster, WebSocketMessage};

pub struct SubstitutionService {
    repo: SubstitutionRepository,
    notifications: NotificationService,
    broadcaster: Arc<Broadcaster>,
}

impl SubstitutionService {
    pub fn new(
        repo: SubstitutionRepository,
        notifications: NotificationService,
        broadcaster: Arc<Broadcaster>,
    ) -> Self {
        Self {
            repo,
            notifications,
            broadcaster,
        }
    }

    pub async fn request_substitution(&self, timetable_entry_id: Uuid) -> AppResult<Substitution> {
        let substitution = Substitution {
            id: Uuid::new_v4(),
            timetable_entry_id,
            substituting_teacher_id: None,
            status: SubstitutionStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = self.repo.create(substitution).await?;
        self.notifications
            .send_substitution_request_notification(result.id);

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "SUBSTITUTION_REQUESTED".to_string(),
            payload: json!({ "id": result.id }),
        });

        Ok(result)
    }

    pub async fn accept_substitution(&self, substitution_id: Uuid, teacher_id: Uuid) -> AppResult<Substitution> {
        let mut substitution = self.repo.find_by_id(substitution_id).await?
            .ok_or(AppError::NotFound)?;

        if substitution.status != SubstitutionStatus::Pending {
            return Err(AppError::BadRequest("Substitution request is not pending".to_string()));
        }

        substitution.substituting_teacher_id = Some(teacher_id);
        substitution.status = SubstitutionStatus::Accepted;
        substitution.updated_at = Utc::now();

        let result = self.repo.update(substitution).await?;
        self.notifications
            .send_substitution_accepted_notification(result.id);

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "SUBSTITUTION_ACCEPTED".to_string(),
            payload: json!({ "id": result.id }),
        });

        Ok(result)
    }

    pub async fn reject_substitution(&self, substitution_id: Uuid) -> AppResult<Substitution> {
        let mut substitution = self.repo.find_by_id(substitution_id).await?
            .ok_or(AppError::NotFound)?;

        if substitution.status != SubstitutionStatus::Pending {
            return Err(AppError::BadRequest("Substitution request is not pending".to_string()));
        }

        substitution.status = SubstitutionStatus::Rejected;
        substitution.updated_at = Utc::now();

        let result = self.repo.update(substitution).await?;
        self.notifications
            .send_substitution_rejected_notification(result.id);

        self.broadcaster.broadcast(WebSocketMessage {
            event_type: "SUBSTITUTION_REJECTED".to_string(),
            payload: json!({ "id": result.id }),
        });

        Ok(result)
    }

    pub async fn get_substitution(&self, id: Uuid) -> AppResult<Option<Substitution>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_substitutions(&self) -> AppResult<Vec<Substitution>> {
        self.repo.find_all().await
    }
}
