use sea_orm::*;
use tracing::instrument;

use ::entity::UserDump;
use ::entity::UserDumpColumn;
use ::entity::UserDumpModel;

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
