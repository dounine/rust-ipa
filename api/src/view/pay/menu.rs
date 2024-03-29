use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use serde_json::json;
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;

#[get("menu")]
#[instrument(skip(state))]
async fn menu(state: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let (menus, _) = service::pay_menu::query_by_page::query_by_page(&state.conn, 0, 100).await?;
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
