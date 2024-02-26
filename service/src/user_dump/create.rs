use sea_orm::*;
use tracing::instrument;

use ::entity::app::AppCountry;
use ::entity::UserDumpActiveModel;

#[instrument(skip(conn))]
pub async fn create(
    conn: &DatabaseTransaction,
    country: AppCountry,
    app_id: &str,
    version: &str,
    user_id: i32,
) -> Result<(), DbErr> {
    UserDumpActiveModel {
        country: Set(country),
        app_id: Set(app_id.to_string()),
        version: Set(version.to_string()),
        user_id: Set(user_id),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map(|_| ())
}
