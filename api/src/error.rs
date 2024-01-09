use actix_web::{HttpResponse, ResponseError};
use crate::response::resp_fail;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Msg(String),
    #[error("db_error: {0}")]
    DbError(#[from] migration::DbErr),
    #[error("token_error: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(resp_fail(self.to_string()))
    }
}