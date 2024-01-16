use actix_web::{HttpResponse, patch};
use actix_web::web::{Data, Json, scope, ServiceConfig};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use entity::app::AppCountry;
use entity::dump::DumpStatus;
use migration::sea_orm::TransactionTrait;

use crate::base::error::MyError;
use crate::base::response::resp_ok_empty;
use crate::base::state::AppState;
use crate::base::token::AdminUserData;

#[derive(Serialize, Deserialize, Debug)]
struct DumpFinishParam {
    app_id: String,
    country: AppCountry,
    version: String,
}

#[patch("/dump_finish")]
#[instrument(skip(state))]
async fn dump_finish(
    state: Data<AppState>,
    _admin_user_data: AdminUserData,
    query: Json<DumpFinishParam>,
) -> Result<HttpResponse, MyError> {
    let DumpFinishParam {
        app_id,
        country,
        version,
    } = query.into_inner();
    service::admin::dump::change_status(&state.conn, country, app_id, version, DumpStatus::Done)
        .await?;
    Ok(resp_ok_empty().into())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/admin/app").service(dump_finish));
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use actix_web::web::{Data, scope};
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
            .service(scope("/admin/app").service(super::dump_finish))
            .app_data(Data::new(AppState::new().await));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::patch()
            .uri("/admin/app/dump_finish")
            .set_json(DumpFinishParam {
                country: AppCountry::Cn,
                app_id: "1".to_string(),
                version: "1.0.0".to_string(),
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
