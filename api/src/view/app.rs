use crate::error::MyError;
use crate::response::{resp_list, resp_ok, resp_ok_empty};
use crate::state::AppState;
use crate::view::base::deserialize_strings_split;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::app::{AppCountry, AppPlatform};
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
    )
    .map_err(|e| MyError::Msg(e.to_string()))?;
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
    let versions =
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
                let version_size = versions
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

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/apps")
            .service(lists)
            .service(create)
            .service(search),
    );
}
