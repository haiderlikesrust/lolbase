use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("An error has occured in the database.")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Unsupported media provided.")]
    UnsupportedMedia
}

#[derive(Serialize)]
pub struct ErrorBody {
    error: String,
    status_code: u16,
}
impl IntoResponse for ErrorBody {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            ApiError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An error has occured in the database."),
            ),
            ApiError::UnsupportedMedia => (
                StatusCode::BAD_GATEWAY,
                format!("Unsupported media provided.")
            )
        };

        let body = ErrorBody {
            error: error_msg,
            status_code: status.as_u16(),
        };

        (status, body).into_response()
    }
}