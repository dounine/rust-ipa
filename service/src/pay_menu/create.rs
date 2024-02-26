use sea_orm::*;
use tracing::instrument;

use ::entity::PayMenuActiveModel;
use ::entity::PayMenuModel;

#[instrument(skip(conn))]
pub async fn create(
    conn: &DbConn,
    form_data: PayMenuModel,
) -> Result<PayMenuModel, DbErr> {
    let mode = PayMenuActiveModel {
        money: Set(form_data.money),
        coin: Set(form_data.coin),
        ..Default::default()
    };
    mode.insert(conn).await
}
