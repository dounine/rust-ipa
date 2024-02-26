use actix_web::web::{scope, ServiceConfig};

use crate::pay_record::{balance, records, transfer};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/coin")
            .service(balance::balance)
            .service(records::records)
            .service(transfer::transfer),
    );
}