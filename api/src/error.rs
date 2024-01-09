use actix_web::{HttpResponse, ResponseError};
use migration::DbErr;
use crate::response::resp_fail;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Msg(String),
    #[error("DbError: {0}")]
    DbError(#[from] DbErr),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(resp_fail(self.to_string()))
    }
}