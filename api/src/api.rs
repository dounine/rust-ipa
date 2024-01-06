use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer};
use actix_web::dev::Service;
use actix_web::HttpMessage;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::web::ServiceConfig;
use clap::Parser;
use listenfd::ListenFd;
use tracing::level_filters::LevelFilter;
use tracing::{info};
use tracing_actix_web::{RootSpan, TracingLogger};
use migration::{Migrator, MigratorTrait};
use crate::limit::RequestLimit;
use crate::span::DomainRootSpanBuilder;
use crate::state::AppState;

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

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let args = Args::parse();
    let file_appender = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();


    // let sub = tracing_subscriber::fmt()
    //     .with_max_level(args.log)
    //     .with_line_number(true)
    //     .event_format(format);
    // if args.release {
    //     sub.with_writer(non_blocking) //正式环境使用
    //         .with_ansi(false)
    //         .init();
    // } else {
    //     sub.init();
    // }
    dotenvy::dotenv().ok();
    let app_state = actix_web::web::Data::new(AppState::new().await);
    let governor_conf = GovernorConfigBuilder::default()
        .key_extractor(RequestLimit::new())
        .per_second(3)
        .burst_size(10)
        .finish()
        .unwrap();
    Migrator::up(&app_state.conn, None).await.unwrap();
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
            .configure(init_router)
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

fn init_router(cfg: &mut ServiceConfig) {
    cfg.configure(crate::user::configure);
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
