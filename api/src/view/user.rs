use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query, scope, ServiceConfig};
use tracing::instrument;
use tracing::log::debug;
use crate::error::MyError;
use crate::response::{list, ok};
use crate::state::AppState;
use crate::view::base::PageOptions;

#[get("")]
#[instrument(skip(state))]
async fn user_list(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, MyError> {
    debug!("进去store查询数据中...");
    let page = page.format();
    service::user::list_user(&state.pool, page.offset, page.limit)
        .await
        .map(|(l, total)| list(l, total))
        .map(|users| HttpResponse::Ok().json(users))
        .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(user_list)
    );
}