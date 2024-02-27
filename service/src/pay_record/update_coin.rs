use sea_orm::*;
use tracing::instrument;

use ::entity::pay_record::PayRecordType;
use ::entity::PayRecordActiveModel;

/// 用户金币变动
#[instrument(skip(conn))]
pub async fn update_coin(
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
    .map(|_| ())
}
