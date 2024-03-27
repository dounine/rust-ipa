use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};
use entity::app::AppCountry;
use serde_json::json;
use tokio::try_join;
use tracing::instrument;

#[get("/{country}/{app_id}/{version}/version_info")]
#[instrument(skip(state))]
async fn version(
    state: Data<AppState>,
    query: Path<(AppCountry, String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (country, app_id,version) = query.into_inner();
    let (app_info, app_version) = try_join!(
        service::app::find_by_appid::find_by_appid(&state.conn, country, app_id.as_str()),
        service::app_version::find_by_appid_and_version::find_by_appid_and_version(
            &state.conn,
            country,
            app_id.as_str(),
            version.as_str(),
        ),
    )?;
    let app_version = app_version
        .map(|v| {
            json!({
                "version": v.version,
                "size": util::file::byte_format(v.size),
                "time": util::time::time_format(v.created_at),
                "created_at": v.created_at,
            })
        });
    Ok(resp_ok(json!({
        "info": app_info,
        "version": app_version
    }))
    .into())
}
