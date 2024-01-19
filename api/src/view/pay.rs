use actix_web::http::StatusCode;
use actix_web::web::{scope, Bytes, Data, Json, Path, ServiceConfig};
use actix_web::{post, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use service::error::ServiceError;
use tracing::{debug, error, instrument};
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo, WechatPayDecodeData, WechatPayNotify};
use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
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

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/pay")
            .service(wechat_notify)
            .service(wechat_pay_order),
    );
}
