use sea_orm::*;
use tracing::instrument;

use ::entity::PayRecord;
use ::entity::PayRecordColumn;
use ::entity::PayRecordModel;

/// 用户金币记录
#[instrument(skip(conn))]
pub async fn query_by_user_and_page(
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
