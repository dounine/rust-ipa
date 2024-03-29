use sea_orm::*;
use tracing::instrument;

use ::entity::PayMenu;
use ::entity::PayMenuColumn;
use ::entity::PayMenuModel;

#[instrument(skip(conn))]
pub async fn query_by_page(
    conn: &DbConn,
    offset: u64,
    limit: u64,
) -> Result<(Vec<PayMenuModel>, u64), DbErr> {
    let paginator = PayMenu::find()
        .order_by_desc(PayMenuColumn::CreatedAt)
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}

#[cfg(test)]
mod tests {
    use std::env;

    use sea_orm::Database;
    use tracing::info;

    #[tokio::test]
    async fn test_infos() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_line_number(true)
            .init();
        info!("test_query_user");
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let conn = Database::connect(database_url)
            .await
            .expect("Cannot connect to database");
        let (lists, total) = super::query_by_page(&conn, 0, 10).await.unwrap();
        assert_eq!(lists.len(), 1);
        assert_eq!(total, 1);
    }
}
