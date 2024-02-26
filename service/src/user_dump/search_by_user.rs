use sea_orm::*;
use tracing::instrument;

use ::entity::app::AppCountry;
use ::entity::UserDump;
use ::entity::UserDumpColumn;
use ::entity::UserDumpModel;

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
