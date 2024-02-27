use crate::admin::dump::change_status;
use actix_web::web::{scope, ServiceConfig};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/admin/dump").service(change_status::change_status));
}
