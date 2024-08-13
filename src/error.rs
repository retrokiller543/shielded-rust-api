use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use shielded_rust::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("An unspecified internal error occurred: {0}")]
    InternalError(#[from] anyhow::Error),
    #[error("Unauthorized request")]
    Unauthorized,
}

impl From<Error> for BackendError {
    fn from(e: Error) -> Self {
        BackendError::InternalError(e.into())
    }
}

impl ResponseError for BackendError {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
