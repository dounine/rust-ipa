use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Path, scope, ServiceConfig};
use tracing::instrument;
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo};
use wechat_pay_rust_sdk::pay::WechatPay;

use crate::base::error::MyError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;

#[post("/{id}")]
#[instrument(skip(state))]
async fn pay(state: Data<AppState>, _id: Path<String>) -> Result<HttpResponse, MyError> {
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
