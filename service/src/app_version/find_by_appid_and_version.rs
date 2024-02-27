use sea_orm::*;
use tracing::instrument;

use ::entity::app::AppCountry;
use ::entity::AppVersion;
use ::entity::AppVersionColumn;
use ::entity::AppVersionModel;

#[instrument(skip(conn))]
pub async fn find_by_appid_and_version(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
    version: &str,
) -> Result<Option<AppVersionModel>, DbErr> {
    AppVersion::find()
        .filter(
            AppVersionColumn::Country
                .eq(country)
                .and(AppVersionColumn::AppId.eq(app_id))
                .and(AppVersionColumn::Version.eq(version)),
        )
        .order_by_desc(AppVersionColumn::CreatedAt)
        .one(conn)
        .await
}
