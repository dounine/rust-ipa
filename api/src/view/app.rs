use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json, Query, scope, ServiceConfig};
use serde::Deserialize;
use tracing::instrument;
use entity::app::AppCountry;
use crate::error::MyError;
use crate::response::{resp_list, resp_ok, resp_ok_empty};
use crate::state::AppState;
use crate::view::base::PageOptions;
use crate::view::base::deserialize_strings_split;

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
async fn create(state: Data<AppState>, form: Json<entity::AppModel>) -> Result<HttpResponse, MyError> {
    service::app::create(&state.conn, form.into_inner())
        .await
        .map(|r| resp_ok_empty())
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

#[get("/search")]
#[instrument(skip(state))]
async fn search(state: Data<AppState>, query: Query<SearchAppParam>) -> Result<HttpResponse, MyError> {
    service::app::search_by_name(&state.conn, &query.country, &query.name)
        .await
        .map(|r| resp_ok(r))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/apps")
            .service(lists)
            .service(create)
            .service(search)
    );
}