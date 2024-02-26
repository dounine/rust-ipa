use actix_web::web::{scope, ServiceConfig};

use crate::admin::app::dump_change_status;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/admin/app").service(dump_change_status::dump_change_status));
}
