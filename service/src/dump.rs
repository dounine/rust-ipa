use ::entity::app::AppCountry;
use ::entity::Dump;
use ::entity::DumpActiveModel;
use ::entity::DumpColumn;
use ::entity::DumpModel;
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn create(conn: &DatabaseTransaction, data: DumpModel) -> Result<(), DbErr> {
    let data = DumpActiveModel {
        country: Set(data.country),
        app_id: Set(data.app_id),
        version: Set(data.version),
        size: Set(data.size),
        name: Set(data.name),
        icon: Set(data.icon),
        link: Set(data.link),
        bundle_id: Set(data.bundle_id),
        price: Set(data.price),
        status: Set(data.status),
        ..Default::default()
    };
    Dump::insert(data)
        .on_conflict(
            OnConflict::columns([DumpColumn::Country, DumpColumn::AppId, DumpColumn::Version])
                .do_nothing()
                .to_owned(),
        )
        .on_empty_do_nothing()
        .exec(conn)
        .await
        .map(|x| ())
}

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

#[instrument(skip(conn))]
pub async fn search_info(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
    version: &str,
) -> Result<Option<DumpModel>, DbErr> {
    Dump::find()
        .filter(
            DumpColumn::Country
                .eq(country)
                .and(DumpColumn::AppId.eq(app_id))
                .and(DumpColumn::Version.eq(version)),
        )
        .one(conn)
        .await
}
