use actix_web::{get, HttpResponse};
use actix_web::web::{Data, scope, ServiceConfig};
use tracing::instrument;

use crate::base::error::MyError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[get("/balance")]
#[instrument(skip(state))]
async fn balance(state: Data<AppState>, user_data: UserData) -> Result<HttpResponse, MyError> {
    let coin_balance = service::pay_record::user_coin_sum(&state.conn, user_data.id)
        .await?
        .unwrap_or(0);
    Ok(resp_ok(coin_balance).into())
}


pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/coin")
            .service(balance),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use actix_web::web::{Data, scope};
    use tracing::debug;

    use crate::base::state::AppState;
    use crate::pay_record::balance;

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
}