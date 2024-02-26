use actix_web::web::{Data, Path};
use actix_web::{patch, HttpResponse};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[patch("/transfer/{coin}/{to_user_id}")]
#[instrument(skip(state))]
async fn transfer(
    state: Data<AppState>,
    user_data: UserData,
    params: Path<(u32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (coin, to_user_id) = params.into_inner();
    if user_data.id == to_user_id {
        return Err(ApiError::msg("不能给自己转帐".to_string()));
    }
    service::user::find_user_by_id(&state.conn, to_user_id)
        .await?
        .ok_or_else(|| ApiError::msg("转帐目标用户不存在".to_string()))?;
    service::pay_record::transfer::transfer(&state.conn, user_data.id, to_user_id, coin).await?;
    Ok(resp_ok_empty().into())
}
