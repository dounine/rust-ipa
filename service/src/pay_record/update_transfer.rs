use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use tracing::instrument;

use ::entity::pay_record::PayRecordType;
use ::entity::PayRecord;
use ::entity::PayRecordActiveModel;
use ::entity::PayRecordColumn;

#[instrument(skip(conn))]
pub async fn update_transfer(
    conn: &DbConn,
    from_user_id: i32,
    to_user_id: i32,
    coin: u32,
) -> Result<(), DbErr> {
    let tx = conn.begin().await?;
    let user_blance: i64 = PayRecord::find()
        .select_only()
        .column_as(PayRecordColumn::Coin.sum(), "coin_sum")
        .filter(PayRecordColumn::UserId.eq(from_user_id))
        .into_tuple()
        .one(&tx)
        .await?
        .unwrap_or(0);
    if user_blance < coin as i64 {
        return Err(DbErr::Custom("余额不足".to_string()));
    }
    let from_active = PayRecordActiveModel {
        user_id: Set(from_user_id),
        coin: Set(-(coin as i32)),
        record_type: Set(PayRecordType::Give),
        ..Default::default()
    };
    let to_active = PayRecordActiveModel {
        user_id: Set(to_user_id),
        coin: Set(coin as i32),
        record_type: Set(PayRecordType::Receive),
        ..Default::default()
    };
    PayRecord::insert_many([from_active, to_active])
        .on_conflict(
            OnConflict::column(PayRecordColumn::Id)
                .do_nothing()
                .to_owned(),
        )
        .exec(&tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
