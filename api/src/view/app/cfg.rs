use actix_web::web::{scope, ServiceConfig};

use crate::app::{add, hots, latest_version, list, news, search, version, versions};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/app")
            .service(list::list)
            .service(add::add)
            .service(version::version)
            .service(versions::versions)
            .service(search::search)
            .service(latest_version::latest_version)
            .service(hots::hots)
            .service(news::news)
    );
}