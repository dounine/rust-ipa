use actix_web::{get, HttpResponse};
use actix_web::web::Data;
use serde_json::json;
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;

#[get("menu")]
#[instrument(skip(state))]
async fn pay_menu(state: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let (menus, _) = service::pay_menu::list_pay_menu(&state.conn, 0, 100).await?;
    let menus = menus
        .into_iter()
        .map(|x| {
            json!( {
                "id": x.id,
                "money": x.money,
                "coin": x.coin,
            })
        })
        .collect::<Vec<_>>();
    Ok(resp_ok(menus).into())
}