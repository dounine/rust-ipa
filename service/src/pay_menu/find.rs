use sea_orm::*;
use tracing::instrument;

use ::entity::PayMenu;
use ::entity::PayMenuColumn;
use ::entity::PayMenuModel;

#[instrument(skip(conn))]
pub async fn find(conn: &DbConn, id: i32) -> Result<Option<PayMenuModel>, DbErr> {
    PayMenu::find()
        .filter(PayMenuColumn::Id.eq(id))
        .one(conn)
        .await
}
