use actix_web::web::{scope, Data, Json, ServiceConfig};
use actix_web::{patch, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::base::error::ApiError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;
use crate::base::token::AdminUserData;
use entity::app::AppCountry;
use entity::dump::DumpStatus;

#[derive(Serialize, Deserialize, Debug)]
struct DumpFinishParam {
    app_id: String,
    country: AppCountry,
    version: String,
    status: DumpStatus,
}

/// 修改应用dump状态为完成
#[patch("/change_status")]
#[instrument(skip(state))]
async fn dump_change_status(
    state: Data<AppState>,
    _admin_user_data: AdminUserData,
    query: Json<DumpFinishParam>,
) -> Result<HttpResponse, ApiError> {
    let DumpFinishParam {
        app_id,
        country,
        version,
        status,
    } = query.into_inner();
    service::admin::dump::change_status(&state.conn, country, app_id, version, status).await?;
    Ok(resp_ok_empty().into())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/admin/app").service(dump_change_status));
}

#[cfg(test)]
mod tests {
    use actix_web::web::{scope, Data};
    use actix_web::{test, App};
    use tracing::debug;

    use entity::app::AppCountry;

    use crate::admin::app::DumpFinishParam;
    use crate::base::state::AppState;

    #[tokio::test]
    async fn test_dump_finish() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let app = App::new()
            .service(scope("/admin/app").service(super::dump_change_status))
            .app_data(Data::new(AppState::new().await));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::patch()
            .uri("/admin/app/change_status")
            .set_json(DumpFinishParam {
                country: AppCountry::Cn,
                app_id: "1".to_string(),
                version: "1.0.0".to_string(),
                status: entity::dump::DumpStatus::Done,
            })
            .insert_header(("Authorization", "Bearer 1"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();
        debug!("body {}", body);
        assert_eq!(status, 200);
    }
}
