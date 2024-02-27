use sea_orm::*;
use tracing::instrument;

use ::entity::app::AppCountry;
use ::entity::AppVersion;
use ::entity::AppVersionColumn;
use ::entity::AppVersionModel;

#[instrument(skip(conn))]
pub async fn query_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Vec<AppVersionModel>, DbErr> {
    AppVersion::find()
        .filter(
            AppVersionColumn::Country
                .eq(country)
                .and(AppVersionColumn::AppId.eq(app_id)),
        )
        .order_by_desc(AppVersionColumn::CreatedAt)
        .all(conn)
        .await
}