use sea_orm::*;
use tracing::instrument;

use ::entity::User;
use ::entity::UserColumn;
use ::entity::UserModel;

#[instrument(skip(conn))]
pub async fn list(
    conn: &DbConn,
    offset: u64,
    limit: u64,
) -> Result<(Vec<UserModel>, u64), DbErr> {
    let paginator = User::find()
        .order_by_desc(UserColumn::Id)
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}
