use actix_web::web::{scope, ServiceConfig};

use crate::app::{add, hots, latest_version, list, search, versions};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/app")
            .service(list::list)
            .service(add::add)
            .service(versions::versions)
            .service(search::search)
            .service(latest_version::latest_version)
            .service(hots::hots)
    );
}