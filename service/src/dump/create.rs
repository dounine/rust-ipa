use sea_orm::*;
use sea_orm::sea_query::OnConflict;
use tracing::instrument;

use ::entity::Dump;
use ::entity::DumpActiveModel;
use ::entity::DumpColumn;
use ::entity::DumpModel;

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
        .map(|_| ())
}
