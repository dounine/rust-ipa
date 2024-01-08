use std::fmt::Debug;
use ::entity::app_version::NewModel;
use ::entity::app::AppCountry;
use ::entity::AppVersionColumn;
use ::entity::AppVersionModel;
use ::entity::AppVersionActiveModel;
use ::entity::AppVersion;
use sea_orm::*;
use sea_orm::sea_query::ArrayType;
use sea_orm::sea_query::ArrayType::String;
use sea_orm::sea_query::TableRef::SubQuery;
use tracing::instrument;

struct AppIds(Vec<String>);

impl From<AppIds> for Value {
    fn from(value: AppIds) -> Self {
        Value::Array(ArrayType::String, value.0.into_iter().collect())
    }
}

#[instrument(skip(conn))]
pub async fn infos(
    conn: &DbConn,
    country: &AppCountry,
    app_ids: &Vec<String>,
) -> Result<Vec<NewModel>, DbErr> {
    AppVersion::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
             SELECT
                a.country,
                a.app_id,
               (
                    SELECT
                        b.version
                    FROM app_version AS b
                    WHERE
                        a.app_id = b.app_id AND a.country = b.country
                    ORDER BY created_at DESC
                    LIMIT 1
               ) AS version,
               (
                    SELECT
                        b.size
                    FROM app_version AS b
                    WHERE
                        a.app_id = b.app_id AND a.country = b.country
                    ORDER BY created_at DESC
                    LIMIT 1
                ) AS size
            FROM app_version AS a
            WHERE
                a.country = $1 AND app_id IN ($2)
            GROUP BY a.country, a.app_id
            "#,
            [country.clone().into(), AppIds(app_ids.clone()).into()],
        ))
        .into_model::<NewModel>()
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
        let app_ids = vec!["1".to_owned(), "2".to_owned()];
        let lists = super::infos(&conn, &AppCountry::Cn, &app_ids).await.unwrap();
        assert_eq!(lists.len(), 1);
    }
}