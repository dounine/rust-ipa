use actix_web::web::{scope, ServiceConfig};
use serde::{Deserialize, Serialize};

use migration::sea_orm::TransactionTrait;

use crate::app::{create, dump, latest_version, lists, search, versions};

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use actix_web::web::{Data, scope};
    use tracing::debug;

    use entity::app::AppCountry;
    use crate::app::dump;

    use crate::base::state::AppState;

    #[tokio::test]
    async fn test_dump() {
        std::env::set_var("RUST_LOG", "debug");
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let app = App::new()
            .service(scope("/app").service(super::dump_app))
            .app_data(Data::new(AppState::new().await));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/app/dump")
            .set_json(dump::DumpParam {
                country: AppCountry::Cn,
                app_id: "1".to_string(),
                version: "1.0.0".to_string(),
                name: "微信".to_string(),
                bundle_id: "com.tencent.wechat".to_string(),
                icon: "https://baidu.com".to_string(),
                link: "https://baidu.com".to_string(),
                genres: "社交".to_string(),
                size: 1024 * 10,
                price: 0,
                content: Some("".to_string()),
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
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/app")
            .service(lists::lists)
            .service(create::create)
            .service(versions::versions)
            .service(search::search)
            .service(latest_version::latest_version)
            .service(dump::dump_app),
    );
}