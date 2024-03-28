use crate::app::news;
use actix_web::web::{Data, Path, Query};
use actix_web::{get, HttpResponse};
use serde_json::json;
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::{resp_fail, resp_ok};
use crate::base::state::AppState;
use crate::view::base::PageOptions;

/// 热门应用
#[get("/classify/{id}")]
#[instrument(skip(state))]
async fn classify(
    state: Data<AppState>,
    query: Path<String>,
    page: Query<PageOptions>,
) -> Result<HttpResponse, ApiError> {
    let page = page.into_inner();
    if page.limit > 42 {
        return Ok(resp_fail("limit must be less than 42".to_string()).into());
    }
    let id = query.into_inner();
    if id == "hots" {
        return service::app::query_hots::query_hots(&state.conn, page.offset, page.limit)
            .await
            .map(|l| {
                resp_ok({
                    json!({
                        "info":{
                            "title": "热门应用",
                            "icon":"icon-app",
                        },
                        "list": l,
                        "next": true,
                    })
                })
            })
            .map(|l| HttpResponse::Ok().json(l))
            .map(Ok)?;
    } else if id == "news" {
        return service::dump::query_news::query_news(&state.conn, page.offset, page.limit)
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
            .map(Ok)?;
    } else if id == "assist" {
    } else {
    }
    service::app::query_hots::query_hots(&state.conn, page.offset, 30)
        .await
        .map(|l| resp_ok(l))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}
