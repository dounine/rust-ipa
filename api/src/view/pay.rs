use actix_web::http::StatusCode;
use actix_web::web::{scope, Bytes, Data, Json, Path, ServiceConfig};
use actix_web::{get, post, HttpRequest, HttpResponse};
use cached::proc_macro::{cached, io_cached};
use cached::IOCachedAsync;
use cached::{AsyncRedisCache, Cached, TimedSizedCache};
use serde::{Deserialize, Serialize};
use service::sea_orm::DbConn;
use std::time::Duration;
use tracing::{debug, error, instrument};
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo, WechatPayDecodeData, WechatPayNotify};
use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

use crate::base::error::ApiError;
use crate::base::response::{resp_ok, resp_ok_empty};
use crate::base::state::AppState;

#[post("/wechat/notify")]
#[instrument(skip(state))]
async fn wechat_notify(
    state: Data<AppState>,
    data: Json<WechatPayNotify>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();
    let nonce = data.resource.nonce;
    let ciphertext = data.resource.ciphertext;
    let associated_data = data
        .resource
        .associated_data
        .ok_or(ApiError::msg("associated_data is none".to_string()))?;
    dotenvy::dotenv().ok();
    let wechat_pay = WechatPay::from_env();
    let result = wechat_pay
        .decrypt_paydata(
            ciphertext,      //加密数据
            nonce,           //随机串
            associated_data, //关联数据
        )
        .map_err(|e| ApiError::msg(e.to_string()))?;
    debug!("pay notify decrypt result: {:?}", result);
    match service::pay::change_payed_status(&state.conn, result.out_trade_no).await {
        Ok(_) => Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()),
        Err(e) => {
            error!("change_payed_status error: {:?}", e);
            let status500 = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(HttpResponse::Ok().status(status500).finish())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayParams {
    money: u32,
    time: u64,
    sign: String,
}

#[post("/wechat/order")]
#[instrument(skip(state))]
async fn wechat_pay_order(
    state: Data<AppState>,
    data: Json<PayParams>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();

    let mut maps = vec![
        ("money", data.money.to_string()),
        ("time", data.time.to_string()),
    ];
    maps.sort_by(|a, b| a.0.cmp(&b.0));
    let sign_str = maps
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");
    let sign = util::crypto::md5(sign_str);
    if sign != data.sign {
        return Err(ApiError::msg("签名验证失败".to_string()));
    }
    let time = chrono::Local::now().timestamp();
    // 5分钟有效期
    if time - data.time as i64 > 5 * 60 {
        return Err(ApiError::msg("定单失效，请重新创建".to_string()));
    }
    // let wechat_pay = WechatPay::from_env();
    // let _conn = &state.conn;
    // let pay_params = H5Params::new(
    //     "测试支付1分",
    //     "1243243",
    //     1.into(),
    //     H5SceneInfo::new("8.210.234.214", "rust收钱", "https://crates.io"),
    // );
    // wechat_pay.h5_pay(pay_params).await.unwrap();

    Ok(HttpResponse::Ok().json(resp_ok_empty()))
    //     .await
    //     .map(|user| resp_ok(user))
    //     .map(|user| HttpResponse::Ok().json(user))
    //     .map_err(|e| MyError::Msg(e.to_string()))
    //     .map(Ok)?
}

// #[cached(time = 1, key = "String", convert = r#"{|x| x }"#, result = true)]
// async fn only_cached_a_second(key: String) -> Result<String, &'static ApiError> {
//     Ok(key)
// }
#[cached(
    result = true,
    convert = r#"{ s.to_string() }"#,
    create = r#"{ TimedSizedCache::with_size_and_lifespan(3, 3) }"#,
    type = r#"TimedSizedCache<String, String>"#
)]
async fn only_cached_a_second(s: String, conn: &DbConn) -> Result<String, ApiError> {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(s + &now)
}

#[get("/cache/{key}")]
#[instrument(skip(state))]
async fn cache_test(state: Data<AppState>, key: Path<String>) -> Result<HttpResponse, ApiError> {
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
            .service(wechat_notify)
            .service(wechat_pay_order)
            .service(cache_test),
    );
}
