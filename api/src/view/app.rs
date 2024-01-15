use crate::base::error::MyError;
use crate::base::response::{resp_list, resp_ok, resp_ok_empty};
use crate::base::state::AppState;
use crate::base::token::UserData;
use crate::view::base::deserialize_strings_split;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::app::{AppCountry, AppPlatform};
use entity::dump::DumpStatus;
use migration::DbErr;
use serde::{Deserialize, Serialize};
use tokio::try_join;
use tracing::instrument;

#[get("")]
#[instrument(skip(state))]
async fn lists(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, MyError> {
    let page = page.format();
    service::app::list(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| resp_list(l, total))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

#[post("")]
#[instrument(skip(state))]
async fn create(
    state: Data<AppState>,
    form: Json<entity::AppModel>,
) -> Result<HttpResponse, MyError> {
    service::app::create(&state.conn, form.into_inner())
        .await
        .map(|_| resp_ok_empty())
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

#[derive(Deserialize, Debug)]
struct SearchAppParam {
    name: String,
    country: AppCountry,
    #[serde(deserialize_with = "deserialize_strings_split")]
    app_ids: Vec<String>,
}
#[derive(Serialize, Debug)]
struct AppInfo {
    name: String,
    country: AppCountry,
    version: String,
    size: i64,
    des: String,
    icon: String,
    platform: AppPlatform,
    bundle_id: String,
    price: i32,
    genres: String,
    single: bool,
}
#[derive(Serialize, Debug)]
struct SearchApp {
    app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<AppInfo>,
}

/// 搜索应用
#[get("/search")]
#[instrument(skip(state))]
async fn search(
    state: Data<AppState>,
    query: Query<SearchAppParam>,
) -> Result<HttpResponse, MyError> {
    let (search_apps, db_apps) = try_join!(
        service::app::search_by_name(&state.conn, &query.country, query.name.as_str()),
        service::app::search_by_appids(
            &state.conn,
            &query.country,
            query.app_ids.iter().map(|x| x.as_str()).collect()
        )
    )?;
    let mut apps: Vec<String> = vec![];
    search_apps.iter().map(|x| x.app_id.clone()).for_each(|x| {
        if !apps.contains(&x) {
            apps.push(x);
        }
    });
    query.app_ids.iter().for_each(|x| {
        if !apps.contains(&x) {
            apps.push(x.clone());
        }
    });

    let version_list =
        service::app_version::search_by_appids(&state.conn, query.country.clone(), apps.clone())
            .await?;
    let mut app_infos: Vec<SearchApp> = vec![];
    apps.iter().for_each(|appid| {
        match search_apps
            .iter()
            .find(|y| y.app_id == *appid)
            .or_else(|| db_apps.iter().find(|y| y.app_id == *appid))
        {
            None => app_infos.push(SearchApp {
                app_id: appid.clone(),
                info: None,
            }),
            Some(info) => {
                let version_size = version_list
                    .iter()
                    .find(|x| x.app_id == info.app_id)
                    .map(|x| (x.version.clone(), x.size.clone()))
                    .map_or_else(|| ("".to_string(), 0), |x| x);
                app_infos.push(SearchApp {
                    app_id: info.app_id.clone(),
                    info: Some(AppInfo {
                        name: info.name.clone(),
                        country: info.country.clone(),
                        version: version_size.0,
                        size: version_size.1,
                        des: info.des.clone(),
                        icon: info.icon.clone(),
                        platform: info.platform.clone(),
                        bundle_id: info.bundle_id.clone(),
                        price: info.price,
                        genres: info.genres.clone(),
                        single: info.single,
                    }),
                })
            }
        }
    });
    Ok(HttpResponse::Ok().json(resp_ok(app_infos)))
}

/// 查看应用版本
#[get("/{country}/{app_id}/versions")]
#[instrument(skip(state))]
async fn versions(
    state: Data<AppState>,
    query: Path<(AppCountry, String)>,
) -> Result<HttpResponse, MyError> {
    let (country, app_id) = query.into_inner();
    let (app_info, app_versions) = try_join!(
        service::app::search_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::search_by_appid(&state.conn, country, app_id.as_str()),
    )?;
    Ok(resp_ok(serde_json::json!({
        "app_info": app_info,
        "versions": app_versions
    }))
    .into())
}

#[get("/{country}/{app_id}/latest_version")]
#[instrument(skip(state))]
async fn latest_version(
    state: Data<AppState>,
    user_data: UserData,
    query: Path<(AppCountry, String)>,
) -> Result<HttpResponse, MyError> {
    let (country, app_id) = query.into_inner();
    let (app_info, latest_version, app_version_dump, user_dump) = try_join!(
        service::app::search_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::latest_version_by_appid(&state.conn, country, app_id.as_str()),
        service::dump::search_by_appid(&state.conn, country, app_id.as_str()),
        service::user_dump::search_by_user(&state.conn, country, app_id.as_str(), user_data.id),
    )?;
    Ok(resp_ok(serde_json::json!({
        "app_info": app_info,
        "latest_version": latest_version,
        "dump_status": app_version_dump.map(|x|x.status),
        "user_dumped": user_dump.is_some()
    }))
    .into())
}

#[derive(Deserialize, Debug)]
struct DumpParam {
    country: AppCountry,
    appid: String,
    version: String,
    name: String,
    bundle_id: String,
    icon: String,
    genres: String,
    size: i64,
    content: Option<String>, //base64后的json数据结构
}

#[post("dump")]
#[instrument(skip(state))]
async fn dump_app(
    state: Data<AppState>,
    user_data: UserData,
    data: Json<DumpParam>,
) -> Result<HttpResponse, MyError> {
    let data = data.into_inner();
    let user_dump_info = service::user_dump::search_by_user(
        &state.conn,
        data.country.clone(),
        data.appid.as_str(),
        user_data.id,
    )
    .await?;
    if user_dump_info.is_some() {
        return MyError::msg("您已经提交提取请求，请勿重复提取").into();
    }
    let user_dump_today =
        service::user_dump::search_by_user_today(&state.conn, user_data.id).await?;
    if user_dump_today.len() >= 10 {
        return MyError::msg("您今天已经提交了10次提取请求，请明天再来").into();
    }
    let app_version = service::app_version::search_by_appid_and_version(
        &state.conn,
        data.country.clone(),
        data.appid.as_str(),
        data.version.as_str(),
    )
    .await?;
    if app_version.is_none() {
        if let Some(latest_dump_info) = service::dump::search_latest_version_by_appid(
            &state.conn,
            data.country.clone(),
            data.appid.as_str(),
        )
        .await?
        {
            if vec![DumpStatus::UnDump, DumpStatus::Check, DumpStatus::Pay]
                .into_iter()
                .find(|x| x == &latest_dump_info.status)
                .is_some()
            {
                return MyError::msg("此应用无法提取，请提取其它应用。").into();
            }
        }
    }
    let user_coins = service::pay_record::user_coin_sum(&state.conn, user_data.id).await?;
    if user_coins.is_none() || user_coins.unwrap() < 1 {
        return MyError::msg("为防止人机恶意提取、每次提取应用需要0.01个金币噢、请购买后再提取。").into();
    }
    Ok(resp_ok_empty().into())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/app")
            .service(lists)
            .service(create)
            .service(search)
            .service(versions)
            .service(latest_version),
    );
}
