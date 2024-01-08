use std::fmt::Debug;
use ::entity::app::AppCountry;
use ::entity::AppVersionColumn;
use ::entity::AppVersionModel;
use ::entity::AppVersionActiveModel;
use ::entity::AppVersion;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn infos(
    conn: &DbConn,
    country: &AppCountry,
    app_ids: &Vec<String>,
) -> Result<Vec<AppVersionModel>, DbErr> {
    AppVersion::find()
        .filter(
            Condition::all()
                .add(AppVersionColumn::Country.eq(country.clone()))
                .add(AppVersionColumn::AppId.is_in(app_ids.clone()))
        )
        .all(conn)
        .await
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::time::Duration;
    use entity::app::AppCountry;
    use sea_orm::{ConnectOptions, Database};
    use tracing::{info, log};
    #[tokio::test]
    async fn test_infos() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_line_number(true)
            .init();
        info!("test_query_user");
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let conn = Database::connect(database_url).await.expect("Cannot connect to database");
        let app_ids = vec!["1".to_owned(),"2".to_owned()];
        let lists = super::infos(&conn, &AppCountry::Cn, &app_ids).await.unwrap();
        assert_eq!(lists.len(), 1);
    }
}