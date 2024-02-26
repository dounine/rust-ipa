use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};
use serde_json::json;
use tokio::try_join;
use tracing::instrument;

use entity::app::AppCountry;

use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[get("/{country}/{app_id}/latest_version")]
#[instrument(skip(state))]
pub async fn latest_version(
    state: Data<AppState>,
    user_data: UserData,
    query: Path<(AppCountry, String)>,
) -> Result<HttpResponse, ApiError> {
    let (country, app_id) = query.into_inner();
    let (app_info, latest_version, app_version_dump, user_dump) = try_join!(
        service::app::search_by_appid::search_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::latest_version_by_appid::latest_version_by_appid(
            &state.conn,
            country,
            app_id.as_str()
        ),
        service::dump::search_by_appid::search_by_appid(&state.conn, country, app_id.as_str()),
        service::user_dump::search_by_user::search_by_user(&state.conn, country, app_id.as_str(), user_data.id),
    )?;
    Ok(resp_ok(json!({
        "app_info": app_info,
        "latest_version": latest_version,
        "dump_status": app_version_dump.map(|x|x.status),
        "user_dumped": user_dump.is_some()
    }))
    .into())
}
