use uuid::Uuid;
use tracing::info;

#[derive(Clone, Default)]
pub struct NotificationService;

impl NotificationService {
    pub fn new() -> Self {
        Self
    }

    pub fn send_substitution_request_notification(&self, substitution_id: Uuid) {
        info!("Substitution request notification sent for ID: {}", substitution_id);
    }

    pub fn send_substitution_accepted_notification(&self, substitution_id: Uuid) {
        info!("Substitution accepted notification sent for ID: {}", substitution_id);
    }

    pub fn send_substitution_rejected_notification(&self, substitution_id: Uuid) {
        info!("Substitution rejected notification sent for ID: {}", substitution_id);
    }
}
