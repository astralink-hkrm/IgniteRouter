use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum RouterError {
    ProviderUnavailable(String),
    RateLimited(String),
    AuthenticationFailed(String),
    Internal(String),
    BadRequest(String),
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouterError::ProviderUnavailable(msg) => write!(f, "Provider Unavailable: {}", msg),
            RouterError::RateLimited(msg) => write!(f, "Rate Limited: {}", msg),
            RouterError::AuthenticationFailed(msg) => write!(f, "Auth Failed: {}", msg),
            RouterError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            RouterError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
        }
    }
}

impl std::error::Error for RouterError {}

impl IntoResponse for RouterError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RouterError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            RouterError::AuthenticationFailed(msg) => (StatusCode::UNAUTHORIZED, msg),
            RouterError::RateLimited(msg) => (StatusCode::TOO_MANY_REQUESTS, msg),
            RouterError::ProviderUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            RouterError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "type": "igniterouter_error",
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
