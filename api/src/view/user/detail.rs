use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Path};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::base::token::UserData;

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
