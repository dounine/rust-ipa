use ::entity::dump::DumpStatus;
use sea_orm::*;
use tracing::instrument;

use ::entity::Dump;
use ::entity::DumpColumn;
use ::entity::DumpModel;

#[instrument(skip(conn))]
pub async fn query_news(
    conn: &DbConn,
    offset: u64,
    limit: u64,
) -> Result<Vec<DumpModel>, DbErr> {
    Dump::find()
        .filter(
            DumpColumn::Status
                .eq(DumpStatus::Done)
        )
        .order_by_desc(DumpColumn::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await
}