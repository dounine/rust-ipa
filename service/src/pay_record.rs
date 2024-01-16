use ::entity::pay_record::PayRecordType;
use ::entity::PayRecord;
use ::entity::PayRecordActiveModel;
use ::entity::PayRecordColumn;
use ::entity::PayRecordModel;
use sea_orm::sea_query::Expr;
use sea_orm::*;
use tracing::instrument;

/// 查询用户金币余额
#[instrument(skip(conn))]
pub async fn user_coin_sum(conn: &DbConn, user_id: i32) -> Result<Option<i64>, DbErr> {
    //金币总数
    PayRecord::find()
        .select_only()
        .column_as(PayRecordColumn::Coin.sum(), "coin_sum")
        .filter(PayRecordColumn::UserId.eq(user_id))
        .into_tuple()
        .one(conn)
        .await
}

/// 用户金币变动
#[instrument(skip(conn))]
pub async fn user_coin_change(
    conn: &DatabaseTransaction,
    user_id: i32,
    coin: i32,
    record_type: PayRecordType,
) -> Result<(), DbErr> {
    let coin = match record_type {
        PayRecordType::Charge | PayRecordType::Receive => coin,
        PayRecordType::Extract
        | PayRecordType::Download
        | PayRecordType::Give
        | PayRecordType::Refund => -coin,
    };
    PayRecordActiveModel {
        user_id: Set(user_id),
        coin: Set(coin),
        record_type: Set(record_type),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map(|x| ())
}
