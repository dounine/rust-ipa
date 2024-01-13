use crate::error::MyError;
use crate::response::{resp_list, resp_ok, resp_ok_empty};
use crate::state::AppState;
use crate::token;
use crate::token::UserData;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::user::{Model, UserStatus, UserType};
use serde::Deserialize;
use tracing::instrument;
use tracing::log::debug;
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo};
use wechat_pay_rust_sdk::pay::WechatPay;

#[post("/{id}")]
#[instrument(skip(state))]
async fn pay(state: Data<AppState>, id: Path<String>) -> Result<HttpResponse, MyError> {
    let wechat_pay = WechatPay::from_env();
    let pay_params = H5Params::new(
        "测试支付1分",
        "1243243",
        1.into(),
        H5SceneInfo::new("8.210.234.214", "rust收钱", "https://crates.io"),
    );
    wechat_pay.h5_pay(pay_params);

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
