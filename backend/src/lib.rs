pub mod error;
pub mod config;
pub mod oidc;
pub mod models;
pub mod ws;
pub mod api;
pub mod graphql;
pub mod service;
pub mod repository;
pub mod middleware;

pub use error::{AppError, AppResult};

use std::sync::Arc;
use crate::config::Config;
use crate::oidc::OidcClient;
use crate::ws::Broadcaster;

#[derive(Clone)]
pub struct AppState {
    pub broadcaster: Arc<Broadcaster>,
    pub config: Arc<Config>,
    pub oidc_client: Arc<OidcClient>,
}
