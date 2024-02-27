use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Path, Query, scope, ServiceConfig};
use tracing::instrument;
use tracing::log::debug;
use crate::base::error::ApiError;
use crate::base::response::{resp_list, resp_ok};
use crate::base::state::AppState;
use crate::base::token::UserData;
use crate::user::{detail, login};
use crate::view::base::PageOptions;

#[get("")]
#[instrument(skip(state))]
async fn user_list(
    state: Data<AppState>,
    page: Query<PageOptions>,
) -> Result<HttpResponse, ApiError> {
    debug!("进去store查询数据中...");
    let page = page.format();
    service::user::query_by_page::query_by_page(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| resp_list(l, total).into())
        .map(Ok)?
}

#[get("/{id}")]
#[instrument(skip(state))]
async fn user_detail(
    state: Data<AppState>,
    user: UserData,
    id: Path<i32>,
) -> Result<HttpResponse, ApiError> {
    service::user::find_by_id::find_by_id(&state.conn, id.into_inner())
        .await
        .map(|user| resp_ok(user).into())
        .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .service(user_list)
            .service(detail::user_detail)
            .service(login::user_login),
    );
}
