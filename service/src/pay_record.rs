use ::entity::PayRecord;
use ::entity::PayRecordActiveModel;
use ::entity::PayRecordColumn;
use ::entity::PayRecordModel;
use sea_orm::*;
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn user_coin_sum(conn: &DbConn, user_id: i32) -> Result<Option<u64>, DbErr> {
    //金币总数
    PayRecord::find()
        .select_only()
        .column_as(PayRecordColumn::Coin.sum(), "coin_sum")
        .filter(
            PayRecordColumn::UserId
                .eq(user_id)
        )
        .into_tuple()
        .one(conn)
        .await
}
