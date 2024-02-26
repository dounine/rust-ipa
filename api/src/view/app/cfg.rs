use actix_web::web::{scope, ServiceConfig};

use crate::app::{create, dump, latest_version, lists, search, versions};

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