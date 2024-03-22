use actix_web::web::{Data, Query};
use actix_web::{get, HttpResponse};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::view::base::PageOptions;

/// 热门应用
#[get("news")]
#[instrument(skip(state))]
async fn news(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, ApiError> {
    let page = page.format();
    service::dump::query_news::query_news(&state.conn, page.offset, page.limit)
        .await
        .map(|list| {
            list.into_iter()
                .map(|d| service::app::query_hots::AppItem {
                    app_id: d.app_id,
                    country: d.country,
                    version: d.version,
                    size: d.size,
                    icon: d.icon,
                    price: d.price,
                    genres: "".to_owned(),
                    name: d.name,
                })
                .collect::<Vec<_>>()
        })
        .map(|list| resp_ok(list))
        .map(|result| HttpResponse::Ok().json(result))
        .map(Ok)?
}
