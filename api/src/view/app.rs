use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json, Query, scope, ServiceConfig};
use tracing::instrument;
use crate::error::MyError;
use crate::response::{list, ok, ok_empty};
use crate::state::AppState;
use crate::view::base::PageOptions;

#[get("")]
#[instrument(skip(state))]
async fn lists(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, MyError> {
    let page = page.format();
    service::app::list(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| list(l, total))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

#[post("")]
#[instrument(skip(state))]
async fn create(state: Data<AppState>, form: Json<entity::AppModel>) -> Result<HttpResponse, MyError> {
    service::app::create(&state.conn, form.into_inner())
        .await
        .map(|r| ok_empty())
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/apps")
            .service(lists)
            .service(create)
    );
}