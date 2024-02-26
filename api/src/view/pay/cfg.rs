use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Path, scope, ServiceConfig};
use cached::proc_macro::cached;
use cached::TimedSizedCache;
use tracing::instrument;

use service::sea_orm::DbConn;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::pay::{menu, notify, order};

#[cached(
    result = true,
    convert = r#"{ s.to_string() }"#,
    create = r#"{ TimedSizedCache::with_size_and_lifespan(3, 3) }"#,
    type = r#"TimedSizedCache<String, String>"#
)]
async fn only_cached_a_second(s: String, _conn: &DbConn) -> Result<String, ApiError> {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(s + &now)
}

#[get("/cache/{key}")]
#[instrument(skip(_state))]
async fn cache_test(_state: Data<AppState>, key: Path<String>) -> Result<HttpResponse, ApiError> {
    let key = key.into_inner();
    cached_for_string_key(key)
        .await
        .map(|x| resp_ok(x).into())
        .map(Ok)?
}

#[cached(
    result = true,
    key = "String",
    time = 3,
    size = 3,
    convert = r#"{ format!("{}",s) }"#
)]
async fn cached_for_string_key(s: String) -> Result<String, ApiError> {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(s + &now)
}
#[cached(result = true, key = "i32", time = 3, size = 3, convert = r#"{ k }"#)]
async fn cached_for_string_i32(k: i32) -> Result<String, ApiError> {
    Ok(k.to_string())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/pay")
            .service(notify::wechat_notify)
            .service(order::wechat_pay_order)
            .service(cache_test)
            .service(order::png)
            .service(order::watermark)
            .service(menu::pay_menu),
    );
}
