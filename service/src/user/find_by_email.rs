use sea_orm::*;
use tracing::instrument;

use ::entity::User;
use ::entity::UserColumn;
use ::entity::UserModel;

#[instrument(skip(conn))]
pub async fn find_by_email(conn: &DbConn, email: &str) -> Result<Option<UserModel>, DbErr> {
    User::find()
        .filter(UserColumn::Email.eq(email))
        .one(conn)
        .await
}
