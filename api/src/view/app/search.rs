use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query};
use serde::{Deserialize, Serialize};
use tokio::try_join;
use tracing::instrument;

use entity::app::{AppCountry, AppPlatform};

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::view::base::deserialize_strings_split;

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
pub async fn search(
    state: Data<AppState>,
    query: Query<SearchAppParam>,
) -> Result<HttpResponse, ApiError> {
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