use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal error happen {}", 0)]
    InternalError(String),
    #[error("Authorization failed {}", 0)]
    AuthorizationFailed(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::InternalError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            ApiError::AuthorizationFailed(str) => (StatusCode::UNAUTHORIZED, str).into_response(),
        }
    }
}
