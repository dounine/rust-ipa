use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use entity::app::AppCountry;
use entity::dump::DumpStatus;
use entity::pay_record::PayRecordType;
use entity::DumpModel;
use migration::sea_orm::TransactionTrait;

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DumpParam {
    pub country: AppCountry,
    pub app_id: String,
    pub version: String,
    pub name: String,
    pub bundle_id: String,
    pub icon: String,
    pub link: String,
    pub genres: String,
    pub size: i64,
    pub price: i32,
    pub content: Option<String>, //base64后的json数据结构
}

#[post("dump")]
#[instrument(skip(state))]
pub async fn dump_app(
    state: Data<AppState>,
    user_data: UserData,
    data: Json<DumpParam>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();
    let user_dump_info = service::user_dump::find_by_user::find_by_user(
        &state.conn,
        data.country.clone(),
        data.app_id.as_str(),
        user_data.id,
    )
    .await?;
    if user_dump_info.is_some() {
        return ApiError::msg("您已经提交提取请求，请勿重复提取").into();
    }
    let user_dump_today =
        service::user_dump::find_by_user_today::find_by_user_today(&state.conn, user_data.id)
            .await?;
    if user_dump_today.len() >= 10 {
        return ApiError::msg("您今天已经提交了10次提取请求，请明天再来").into();
    }
    let app_version =
        service::app_version::find_by_appid_and_version::find_by_appid_and_version(
            &state.conn,
            data.country.clone(),
            data.app_id.as_str(),
            data.version.as_str(),
        )
        .await?;
    if app_version.is_none() {
        if let Some(latest_dump_info) = service::dump::find::find(
            &state.conn,
            data.country.clone(),
            data.app_id.as_str(),
            data.version.as_str(),
        )
        .await?
        {
            if vec![DumpStatus::UnDump, DumpStatus::Check, DumpStatus::Pay]
                .into_iter()
                .find(|x| x == &latest_dump_info.status)
                .is_some()
            {
                return ApiError::msg("此应用无法提取，请提取其它应用。").into();
            }
        }
    }
    let user_coins = service::pay_record::find_coin_balance::find_coin_balance(&state.conn, user_data.id).await?;
    if user_coins.is_none() || user_coins.unwrap() < 1 {
        //放后面付费率会下降
        return ApiError::msg("为防止人机恶意提取，每次提取应用需要0.01个金币，请购买后再提取。")
            .into();
    }

    let tx = state.conn.begin().await?;
    service::pay_record::update_coin::update_coin(&tx, user_data.id, 1, PayRecordType::Extract)
        .await?;
    service::user_dump::add::add(
        &tx,
        data.country.clone(),
        data.app_id.as_str(),
        data.version.as_str(),
        user_data.id,
    )
    .await?;
    service::dump::add::add(
        &tx,
        DumpModel {
            country: data.country.clone(),
            app_id: data.app_id.clone(),
            version: data.version.clone(),
            name: data.name.clone(),
            icon: data.icon.clone(),
            link: data.link.clone(),
            bundle_id: data.bundle_id.clone(),
            size: data.size,
            price: data.price,
            status: DumpStatus::Waiting,
            created_at: util::time::now(),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(resp_ok_empty().into())
}

#[cfg(test)]
mod tests {
    use actix_web::web::{scope, Data};
    use actix_web::{test, App};
    use tracing::debug;

    use crate::app::dump;
    use entity::app::AppCountry;

    use crate::base::state::AppState;

    #[tokio::test]
    async fn test_dump() {
        std::env::set_var("RUST_LOG", "debug");
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let app = App::new()
            .service(scope("/app").service(super::dump_app))
            .app_data(Data::new(AppState::new().await));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/app/dump")
            .set_json(dump::DumpParam {
                country: AppCountry::Cn,
                app_id: "1".to_string(),
                version: "1.0.0".to_string(),
                name: "微信".to_string(),
                bundle_id: "com.tencent.wechat".to_string(),
                icon: "https://baidu.com".to_string(),
                link: "https://baidu.com".to_string(),
                genres: "社交".to_string(),
                size: 1024 * 10,
                price: 0,
                content: Some("".to_string()),
            })
            .insert_header(("Authorization", "Bearer 1"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();
        debug!("body {}", body);
        assert_eq!(status, 200);
    }
}
