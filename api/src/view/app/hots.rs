use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::{resp_ok};
use crate::base::state::AppState;
use crate::view::base::PageOptions;

/// 热门应用
#[get("hots")]
#[instrument(skip(state))]
async fn hots(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, ApiError> {
    let page = page.format();
    service::app::query_hots::query_hots(&state.conn, page.offset, page.limit)
        .await
        .map(|l| resp_ok(l))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}