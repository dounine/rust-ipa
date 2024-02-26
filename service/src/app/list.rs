use ::entity::App;
use ::entity::AppModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn list(conn: &DbConn, offset: u64, limit: u64) -> Result<(Vec<AppModel>, u64), DbErr> {
    let paginator = App::find().paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}
