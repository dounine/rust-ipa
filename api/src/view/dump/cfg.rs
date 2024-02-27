use actix_web::web::{scope, ServiceConfig};

use crate::app::add;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/dump")
            .service(add::add),
    );
}