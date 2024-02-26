use crate::base::error::ApiError;
use crate::base::response::{resp_list, resp_ok, resp_ok_empty};
use crate::base::state::AppState;
use crate::base::token::UserData;
use crate::view::base::deserialize_strings_split;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::app::{AppCountry, AppPlatform};
use entity::dump::DumpStatus;
use entity::pay_record::PayRecordType;
use entity::DumpModel;
use migration::sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::try_join;
use tracing::instrument;
use crate::app::{dump, latest_version, lists, search};

#[post("")]
#[instrument(skip(state))]
async fn create(
    state: Data<AppState>,
    form: Json<entity::AppModel>,
) -> Result<HttpResponse, ApiError> {
    service::app::create(&state.conn, form.into_inner())
        .await
        .map(|_| resp_ok_empty())
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}
/// 查看应用版本
#[get("/{country}/{app_id}/versions")]
#[instrument(skip(state))]
async fn versions(
    state: Data<AppState>,
    query: Path<(AppCountry, String)>,
) -> Result<HttpResponse, ApiError> {
    let (country, app_id) = query.into_inner();
    let (app_info, app_versions) = try_join!(
        service::app::search_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::search_by_appid(&state.conn, country, app_id.as_str()),
    )?;
    Ok(resp_ok(json!({
        "app_info": app_info,
        "versions": app_versions
    }))
    .into())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DumpParam {
    country: AppCountry,
    app_id: String,
    version: String,
    name: String,
    bundle_id: String,
    icon: String,
    link: String,
    genres: String,
    size: i64,
    price: i32,
    content: Option<String>, //base64后的json数据结构
}

#[post("dump")]
#[instrument(skip(state))]
async fn dump_app(
    state: Data<AppState>,
    user_data: UserData,
    data: Json<DumpParam>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();
    let user_dump_info = service::user_dump::search_by_user(
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
        service::user_dump::search_by_user_today(&state.conn, user_data.id).await?;
    if user_dump_today.len() >= 10 {
        return ApiError::msg("您今天已经提交了10次提取请求，请明天再来").into();
    }
    let app_version = service::app_version::search_by_appid_and_version(
        &state.conn,
        data.country.clone(),
        data.app_id.as_str(),
        data.version.as_str(),
    )
    .await?;
    if app_version.is_none() {
        if let Some(latest_dump_info) = service::dump::search_info(
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
    let user_coins = service::pay_record::user_coin_sum(&state.conn, user_data.id).await?;
    if user_coins.is_none() || user_coins.unwrap() < 1 {
        //放后面付费率会下降
        return ApiError::msg("为防止人机恶意提取，每次提取应用需要0.01个金币，请购买后再提取。")
            .into();
    }

    let tx = state.conn.begin().await?;
    service::pay_record::user_coin_change(&tx, user_data.id, 1, PayRecordType::Extract).await?;
    service::user_dump::create(
        &tx,
        data.country.clone(),
        data.app_id.as_str(),
        data.version.as_str(),
        user_data.id,
    )
    .await?;
    service::dump::create(
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
    use crate::base::state::AppState;
    use actix_web::web::{scope, Data};
    use actix_web::{test, App};
    use entity::app::AppCountry;
    use tracing::debug;

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
            .set_json(DumpParam {
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
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/app")
            .service(lists::lists)
            .service(create)
            .service(search::search)
            .service(latest_version::latest_version)
            .service(dump::dump_app),
    );
}