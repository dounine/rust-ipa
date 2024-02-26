use ::entity::app::{AppCountry};
use ::entity::App;
use ::entity::AppColumn;
use ::entity::AppModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn search_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Option<AppModel>, DbErr> {
    App::find()
        .filter(
            AppColumn::Country
                .eq(country)
                .and(AppColumn::AppId.eq(app_id)),
        )
        .one(conn)
        .await
}