use actix_web::web::{scope, Bytes, Data, Json, Path, ServiceConfig};
use actix_web::{post, HttpRequest, HttpResponse};
use tracing::{debug, instrument};
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo, WechatPayDecodeData, WechatPayNotify};
use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;

#[post("/weixin/notify")]
#[instrument(skip(state))]
async fn weixin_notify(
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
    let result: WechatPayDecodeData = wechat_pay
        .decrypt_paydata(
            ciphertext,      //加密数据
            nonce,           //随机串
            associated_data, //关联数据
        )
        .await
        .map_err(|e| ApiError::msg(e.to_string()))?;
    debug!("pay notify decrypt result: {:?}", result);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code":"SUCCESS",
        "message":"成功",
    })))
}
#[post("/weixin/{id}")]
#[instrument(skip(state))]
async fn pay(state: Data<AppState>, _id: Path<String>) -> Result<HttpResponse, ApiError> {
    let wechat_pay = WechatPay::from_env();
    let _conn = &state.conn;
    let pay_params = H5Params::new(
        "测试支付1分",
        "1243243",
        1.into(),
        H5SceneInfo::new("8.210.234.214", "rust收钱", "https://crates.io"),
    );
    wechat_pay.h5_pay(pay_params).await.unwrap();

    Ok(HttpResponse::Ok().json(resp_ok_empty()))
    //     .await
    //     .map(|user| resp_ok(user))
    //     .map(|user| HttpResponse::Ok().json(user))
    //     .map_err(|e| MyError::Msg(e.to_string()))
    //     .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/pay").service(pay));
}
