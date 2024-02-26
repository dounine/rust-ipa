use sea_orm::*;
use tracing::instrument;

use ::entity::User;
use ::entity::UserColumn;
use ::entity::UserModel;

#[instrument(skip(conn))]
pub async fn find_by_username(
    conn: &DbConn,
    user_name: &str,
) -> Result<Option<UserModel>, DbErr> {
    User::find()
        .filter(UserColumn::UserName.eq(user_name))
        .one(conn)
        .await
}
