use ::entity::app::AppCountry;
use ::entity::Dump;
use ::entity::DumpActiveModel;
use ::entity::DumpColumn;
use ::entity::DumpModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn search_by_appid(
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

#[instrument(skip(conn))]
pub async fn search_latest_version_by_appid(
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
        .order_by_desc(DumpColumn::CreatedAt)
        .limit(1)
        .one(conn)
        .await
}
