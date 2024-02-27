use actix_web::{HttpResponse, post};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use tracing::{debug, error, instrument};
use wechat_pay_rust_sdk::model::WechatPayNotify;
use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

use crate::base::error::ApiError;
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
    match service::pay::update_payed_status::update_payed_status(&state.conn, result.out_trade_no).await {
        Ok(_) => Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()),
        Err(e) => {
            error!("change_payed_status error: {:?}", e);
            let status500 = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(HttpResponse::Ok().status(status500).finish())
        }
    }
}