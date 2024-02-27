use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;

#[post("")]
#[instrument(skip(state))]
async fn add(
    state: Data<AppState>,
    form: Json<entity::AppModel>,
) -> Result<HttpResponse, ApiError> {
    service::app::add::add(&state.conn, form.into_inner())
        .await
        .map(|_| resp_ok_empty())
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}
