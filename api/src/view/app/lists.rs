use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_list;
use crate::base::state::AppState;
use crate::view::base::PageOptions;

#[get("")]
#[instrument(skip(state))]
async fn lists(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, ApiError> {
    let page = page.format();
    service::app::list(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| resp_list(l, total))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}