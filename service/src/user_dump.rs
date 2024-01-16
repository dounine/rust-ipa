use ::entity::app::AppCountry;
use ::entity::UserDump;
use ::entity::UserDumpActiveModel;
use ::entity::UserDumpColumn;
use ::entity::UserDumpModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn search_by_user(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
    user_id: i32,
) -> Result<Option<UserDumpModel>, DbErr> {
    UserDump::find()
        .filter(
            UserDumpColumn::Country
                .eq(country)
                .and(UserDumpColumn::AppId.eq(app_id))
                .and(UserDumpColumn::UserId.eq(user_id)),
        )
        .one(conn)
        .await
}

#[instrument(skip(conn))]
pub async fn create(
    conn: &DatabaseTransaction,
    country: AppCountry,
    app_id: &str,
    version: &str,
    user_id: i32,
) -> Result<(), DbErr> {
    UserDumpActiveModel {
        country: Set(country),
        app_id: Set(app_id.to_string()),
        version: Set(version.to_string()),
        user_id: Set(user_id),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map(|_| ())
}

#[instrument(skip(conn))]
pub async fn search_by_user_today(
    conn: &DbConn,
    user_id: i32,
) -> Result<Vec<UserDumpModel>, DbErr> {
    let today = chrono::Local::now()
        .naive_local()
        .date()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    UserDump::find()
        .filter(
            UserDumpColumn::UserId
                .eq(user_id)
                .and(UserDumpColumn::CreatedAt.gt(today)),
        )
        .all(conn)
        .await
}
