use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use service::sea_orm::DbErr;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Serialize, Debug)]
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: i64,
}

pub fn ok<T>(data: T) -> Response<T> {
    Response {
        ok: true,
        err: None,
        data: Some(data),
    }
}

pub fn ok_empty() -> Response<String> {
    Response {
        ok: true,
        err: None,
        data: None,
    }
}

pub fn _list<T>(list: Vec<T>, total: i64) -> Response<ListData<T>> {
    Response {
        ok: true,
        err: None,
        data: Some(ListData {
            list,
            total,
        }),
    }
}

pub fn fail(msg: String) -> Response<String> {
    Response {
        ok: false,
        err: Some(msg),
        data: None,
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Msg(String),
    #[error("DbError: {0}")]
    DbError(#[from] DbErr),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(fail(self.to_string()))
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}