use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};
use entity::app::AppCountry;
use serde_json::json;
use tokio::try_join;
use tracing::instrument;

#[get("/{country}/{app_id}/versions")]
#[instrument(skip(state))]
async fn versions(
    state: Data<AppState>,
    query: Path<(AppCountry, String)>,
) -> Result<HttpResponse, ApiError> {
    let (country, app_id) = query.into_inner();
    let (app_info, app_versions) = try_join!(
        service::app::find_by_appid::find_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::query_by_appid::query_by_appid(&state.conn, country, app_id.as_str()),
    )?;
    let app_versions = app_versions
        .into_iter()
        .map(|v| {
            json!({
                "version": v.version,
                "size": util::file::byte_format(v.size),
                "time": util::time::time_format(v.created_at),
                "created_at": v.created_at,
            })
        })
        .collect::<Vec<_>>();
    Ok(resp_ok(json!({
        "info": app_info,
        "versions": app_versions
    }))
    .into())
}
