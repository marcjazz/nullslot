use async_graphql::ErrorExtensions;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
    #[error("Internal server error")]
    InternalError(#[from] anyhow::Error),
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden", "Forbidden".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found", "Not found".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg),
            AppError::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "unprocessable_entity",
                msg,
            ),
            AppError::InternalError(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal",
                    "Internal server error".to_string(),
                )
            }
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal",
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(ErrorEnvelope {
            error: ErrorBody {
                code: code.to_string(),
                message,
                details: None,
            },
        });

        (status, body).into_response()
    }
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| {
            let code = match self {
                AppError::BadRequest(_) => "BAD_REQUEST",
                AppError::Unauthorized => "UNAUTHORIZED",
                AppError::Forbidden => "FORBIDDEN",
                AppError::NotFound => "NOT_FOUND",
                AppError::Conflict(_) => "CONFLICT",
                AppError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
                AppError::InternalError(_) => "INTERNAL_SERVER_ERROR",
                AppError::DatabaseError(_) => "DATABASE_ERROR",
            };
            e.set("code", code);
        })
    }
}

pub type AppResult<T> = Result<T, AppError>;
