use sea_orm::*;
use tracing::instrument;

use ::entity::User;
use ::entity::UserModel;

#[instrument(skip(conn))]
pub async fn find_by_id(conn: &DbConn, id: i32) -> Result<Option<UserModel>, DbErr> {
    User::find_by_id(id).one(conn).await
}