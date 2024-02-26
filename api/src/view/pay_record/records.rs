use actix_web::web::{Data, Query};
use actix_web::{get, HttpResponse};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_list;
use crate::base::state::AppState;
use crate::base::token::UserData;
use crate::view::base::PageOptions;

#[get("/records")]
#[instrument(skip(state))]
async fn records(
    state: Data<AppState>,
    user_data: UserData,
    page: Query<PageOptions>,
) -> Result<HttpResponse, ApiError> {
    let PageOptions { offset, limit } = page.format();
    service::pay_record::list::list(&state.conn, user_data.id, offset, limit)
        .await
        .map(|(l, total)| resp_list(l, total).into())
        .map(Ok)?
}
