use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use tracing::{debug, debug_span, field, Span};
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};

pub(crate) struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        // let trace_id: String = uuid::Uuid::new_v4().to_string().replace("-", "");
        let url = format!("{} {}", request.method(), request.uri());
        let span = debug_span!("", url, trace_id = field::Empty);
        let _enter = span.enter();
        let trace_id = span.id().unwrap().into_u64();
        span.record("trace_id", trace_id);
        debug!(
            "remote ip {}",
            request.connection_info().peer_addr().unwrap_or("127.0.0.1")
        );
        span.clone()
    }

    fn on_request_end<B: MessageBody>(
        span: Span,
        outcome: &Result<ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}
