use crate::response;
use actix_governor::governor::clock::{Clock, DefaultClock, QuantaInstant};
use actix_governor::governor::NotUntil;
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::dev::ServiceRequest;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};

#[derive(Clone)]
pub(crate) struct RequestLimit;

impl RequestLimit {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl KeyExtractor for RequestLimit {
    type Key = String;
    type KeyExtractionError = SimpleKeyExtractionError<Self::Key>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        req.connection_info()
            .realip_remote_addr() //remote ip
            .map_or(
                Err(SimpleKeyExtractionError::new(
                    "remote ip not found".to_string(),
                )),
                |ip| Ok(ip.to_string()),
            )
    }
    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut response: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait_time = negative
            .wait_time_from(DefaultClock::default().now())
            .as_millis();
        response
            .status(StatusCode::OK)
            .json(response::resp_fail(format!(
                "Too many requests, retry in {} millis",
                wait_time
            )))
    }
}
