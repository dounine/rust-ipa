use ::entity::pay_record::PayRecordType;
use ::entity::PayRecord;
use ::entity::PayRecordActiveModel;
use ::entity::PayRecordColumn;
use ::entity::PayRecordModel;
use sea_orm::sea_query::OnConflict;
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
/// 转帐
#[instrument(skip(conn))]
pub async fn transfer(
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
        .one(conn)
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

/// 用户金币记录
#[instrument(skip(conn))]
pub async fn user_records(
    conn: &DbConn,
    user_id: i32,
    offset: u64,
    limit: u64,
) -> Result<(Vec<PayRecordModel>, u64), DbErr> {
    let paginator = PayRecord::find()
        .filter(PayRecordColumn::UserId.eq(user_id))
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
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
    .map(|_| ())
}
