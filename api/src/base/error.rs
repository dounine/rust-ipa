use actix_web::{HttpResponse, ResponseError};

use crate::base::response::resp_fail;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Msg(String),
    #[error("db_error: {0}")]
    DbError(#[from] migration::DbErr),
    #[error("token_error: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    ServiceError(#[from] service::error::ServiceError),
}

impl ApiError {
    pub fn msg(msg: impl AsRef<str>) -> Self {
        ApiError::Msg(msg.as_ref().to_string())
    }
}

impl From<ApiError> for Result<HttpResponse, ApiError> {
    fn from(value: ApiError) -> Self {
        Err(value)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(resp_fail(self.to_string()))
    }
}
