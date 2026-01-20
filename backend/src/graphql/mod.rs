pub mod query;
pub mod mutation;
pub mod schema;
pub mod types;

pub use schema::{create_schema, AppSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct WorkspaceContext {
    pub workspace_id: Option<Uuid>,
}
