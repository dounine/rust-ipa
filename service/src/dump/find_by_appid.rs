use sea_orm::*;
use tracing::instrument;

use ::entity::app::AppCountry;
use ::entity::Dump;
use ::entity::DumpColumn;
use ::entity::DumpModel;

#[instrument(skip(conn))]
pub async fn find_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Option<DumpModel>, DbErr> {
    Dump::find()
        .filter(
            DumpColumn::Country
                .eq(country)
                .and(DumpColumn::AppId.eq(app_id)),
        )
        .one(conn)
        .await
}