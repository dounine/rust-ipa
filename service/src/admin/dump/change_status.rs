use ::entity::app::AppCountry;
use ::entity::dump::DumpStatus;
use ::entity::Dump;
use ::entity::DumpActiveModel;
use ::entity::DumpColumn;
use sea_orm::*;
use std::fmt::Debug;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn change_status<S>(
    conn: &DbConn,
    country: AppCountry,
    app_id: S,
    version: S,
    status: DumpStatus,
) -> Result<(), DbErr>
where
    S: AsRef<str> + Debug,
{
    let app_id = app_id.as_ref();
    let version = version.as_ref();

    let active_mode = Dump::find()
        .filter(
            DumpColumn::Country
                .eq(country)
                .and(DumpColumn::AppId.eq(app_id))
                .and(DumpColumn::Version.eq(version)),
        )
        .one(conn)
        .await?;
    match active_mode {
        Some(actix_model) => {
            let mut actix_model: DumpActiveModel = actix_model.into();
            actix_model.status = Set(status.to_owned());
            actix_model.update(conn).await.map(|_| ())
        }
        None => Err(DbErr::RecordNotFound("dump info not found".to_string())),
    }
}
