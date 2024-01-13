use crate::base::response::{resp_fail, resp_ok, resp_ok_empty};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use std::ops::Deref;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Msg(String),
    #[error("db_error: {0}")]
    DbError(#[from] migration::DbErr),
    #[error("token_error: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),
}

impl MyError {
    pub fn msg(msg: impl AsRef<str>) -> Self {
        MyError::Msg(msg.as_ref().to_string())
    }
}

impl From<MyError> for Result<HttpResponse, MyError> {
    fn from(value: MyError) -> Self {
        Err(value)
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(resp_fail(self.to_string()))
    }
}
