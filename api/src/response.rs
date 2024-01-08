use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

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
    pub total: u64,
}

pub fn resp_ok<T>(data: T) -> Response<T> {
    Response {
        ok: true,
        err: None,
        data: Some(data),
    }
}

pub fn resp_ok_empty() -> Response<String> {
    Response {
        ok: true,
        err: None,
        data: None,
    }
}

pub fn resp_list<T>(list: Vec<T>, total: u64) -> Response<ListData<T>> {
    Response {
        ok: true,
        err: None,
        data: Some(ListData {
            list,
            total,
        }),
    }
}

pub fn resp_fail(msg: String) -> Response<String> {
    Response {
        ok: false,
        err: Some(msg),
        data: None,
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}