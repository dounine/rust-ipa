use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[get("/balance")]
#[instrument(skip(state))]
async fn balance(state: Data<AppState>, user_data: UserData) -> Result<HttpResponse, ApiError> {
    let coin_balance = service::pay_record::find_coin_balance::find_coin_balance(&state.conn, user_data.id)
        .await?
        .unwrap_or(0);
    Ok(resp_ok(coin_balance).into())
}
