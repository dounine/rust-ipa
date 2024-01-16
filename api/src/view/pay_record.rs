use actix_web::web::{scope, Data, Query, ServiceConfig};
use actix_web::{get, HttpResponse};
use tracing::instrument;

use crate::base::error::MyError;
use crate::base::response::{resp_list, resp_ok};
use crate::base::state::AppState;
use crate::base::token::UserData;
use crate::view::base::PageOptions;

#[get("/balance")]
#[instrument(skip(state))]
async fn balance(state: Data<AppState>, user_data: UserData) -> Result<HttpResponse, MyError> {
    let coin_balance = service::pay_record::user_coin_sum(&state.conn, user_data.id)
        .await?
        .unwrap_or(0);
    Ok(resp_ok(coin_balance).into())
}

#[get("/records")]
#[instrument(skip(state))]
async fn records(
    state: Data<AppState>,
    user_data: UserData,
    page: Query<PageOptions>,
) -> Result<HttpResponse, MyError> {
    let PageOptions { offset, limit } = page.format();
    service::pay_record::user_records(&state.conn, user_data.id, offset, limit)
        .await
        .map(|(l, total)| resp_list(l, total))
        .map(|l| HttpResponse::Ok().json(l))
        .map(Ok)?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/coin").service(balance).service(records));
}

#[cfg(test)]
mod tests {
    use actix_web::web::{scope, Data};
    use actix_web::{test, App};
    use tracing::debug;

    use crate::base::state::AppState;
    use crate::pay_record::{balance, records};

    #[tokio::test]
    async fn test_balance() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let app = App::new()
            .configure(|cfg| {
                cfg.service(scope("/coin").service(balance));
            })
            .app_data(Data::new(AppState::new().await));

        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/coin/balance")
            .insert_header(("Authorization", "Bearer 1"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();
        debug!("body: {}", body);
    }

    #[tokio::test]
    async fn test_records() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let app = App::new()
            .configure(|cfg| {
                cfg.service(scope("/coin").service(records));
            })
            .app_data(Data::new(AppState::new().await));

        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/coin/records")
            .insert_header(("Authorization", "Bearer 1"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();
        debug!("body: {}", body);
    }
}
