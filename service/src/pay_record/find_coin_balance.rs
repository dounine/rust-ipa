use sea_orm::*;
use tracing::instrument;

use ::entity::PayRecord;
use ::entity::PayRecordColumn;

/// 查询用户金币余额
#[instrument(skip(conn))]
pub async fn find_coin_balance(conn: &DbConn, user_id: i32) -> Result<Option<i64>, DbErr> {
    //金币总数
    PayRecord::find()
        .select_only()
        .column_as(PayRecordColumn::Coin.sum(), "coin_sum")
        .filter(PayRecordColumn::UserId.eq(user_id))
        .into_tuple()
        .one(conn)
        .await
}
