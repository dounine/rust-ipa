use std::env;
use std::sync::Mutex;
use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError};
use actix_governor::governor::clock::{Clock, DefaultClock, QuantaInstant};
use actix_governor::governor::NotUntil;
use actix_web::http::StatusCode;
use actix_web::{App, HttpResponse, HttpResponseBuilder, HttpServer};
use actix_web::body::MessageBody;
use actix_web::dev::Service;
use actix_web::HttpMessage;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use clap::Parser;
use listenfd::ListenFd;
use tracing::level_filters::LevelFilter;
use tracing::{debug, debug_span, field, info, Span};
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpan, RootSpanBuilder, TracingLogger};
use migration::{Migrator, MigratorTrait};
use service::sea_orm::{Database, DatabaseConnection};
use crate::response;

#[derive(Debug, Parser)]
#[command(author, version, about = "这是关于信息")]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value = "8080")]
    port: u16,
    #[arg(long, default_value = "debug")]
    log: LevelFilter,
    #[arg(long, default_value = "false")]
    release: bool,
}

pub struct AppState {
    pub users: Mutex<Vec<String>>,
    pub pool: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
            pool: Database::connect(&env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")).await.unwrap(),
        }
    }
}

#[derive(Clone)]
struct RequestLimit;

impl RequestLimit {
    fn new() -> Self {
        Self
    }
}

impl KeyExtractor for RequestLimit {
    type Key = String;
    type KeyExtractionError = SimpleKeyExtractionError<Self::Key>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        req.connection_info()
            .realip_remote_addr()//remote ip
            .map_or(Err(SimpleKeyExtractionError::new("remote ip not found".to_string())), |ip| Ok(ip.to_string()))
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
            .json(response::fail(format!("Too many requests, retry in {} millis", wait_time)))
    }
}

struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        // let trace_id: String = uuid::Uuid::new_v4().to_string().replace("-", "");
        let url = format!("{} {}", request.method(), request.uri());
        let span = debug_span!("",url,trace_id = field::Empty);
        let _enter = span.enter();
        let trace_id = span.id().unwrap().into_u64();
        span.record("trace_id", trace_id);
        debug!("remote ip {}", request.connection_info().peer_addr().unwrap_or("127.0.0.1"));
        span.clone()
    }

    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, actix_web::Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let args = Args::parse();
    let file_appender = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true);

    let sub = tracing_subscriber::fmt()
        .with_max_level(args.log)
        .with_line_number(true)
        .event_format(format);
    if args.release {
        sub.with_writer(non_blocking) //正式环境使用
            .with_ansi(false)
            .init();
    } else {
        sub.init();
    }
    dotenvy::dotenv().ok();
    let app_state = actix_web::web::Data::new(AppState::new().await);
    let governor_conf = GovernorConfigBuilder::default()
        .key_extractor(RequestLimit::new())
        .per_second(3)
        .burst_size(10)
        .finish()
        .unwrap();
    Migrator::up(&app_state.pool, None).await.unwrap();
    let mut listened = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async move {
                    fut
                        .await
                        .map(|mut res| {
                            let trace_id = res
                                .request()
                                .extensions().get::<RootSpan>().unwrap().id().unwrap().into_u64().to_string();
                            res.headers_mut()
                                .insert(HeaderName::from_static("trace_id"), HeaderValue::from_str(&trace_id).unwrap());
                            Ok(res)
                        })?
                }
            })
            .wrap(Governor::new(&governor_conf))
            .app_data(actix_web::web::JsonConfig::default().limit(4096))//json body limit 4kb
            .app_data(app_state.clone())//global state
            .wrap(TracingLogger::<DomainRootSpanBuilder>::new())
    })
        .workers(1);
    let server_url = format!("{}:{}", args.host, args.port);
    server = match listened.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };
    let server_url = format!("http://{:?}", server.addrs().iter().next().unwrap());
    info!("Starting server at {}",server_url);
    server.run().await?;
    Ok(())
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
