use ::entity::app::AppCountry;
use ::entity::app_version::NewModel;
use ::entity::AppVersion;
use ::entity::AppVersionColumn;
use ::entity::AppVersionModel;
use sea_orm::*;
use tracing::instrument;
use util::sql::{Sql, SqlTrait};

#[instrument(skip(conn))]
pub async fn search_by_appids(
    conn: &DbConn,
    country: AppCountry,
    app_ids: Vec<String>,
) -> Result<Vec<NewModel>, DbErr> {
    let arr_size = "ARRAY [".len();
    let app_ids = Value::from(app_ids).to_string();
    let app_ids = &app_ids[arr_size..app_ids.len() - 1]; //ARRAY ['1','2'] => '1','2'

    let sql = Sql::from(format!(
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
                a.country = $1 AND app_id IN ({})
            GROUP BY a.country, a.app_id
            "#,
        app_ids
    ));

    AppVersion::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql.compress(),
            [country.into()],
        ))
        .into_model::<NewModel>()
        .all(conn)
        .await
}
#[instrument(skip(conn))]
pub async fn search_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Vec<AppVersionModel>, DbErr> {
    AppVersion::find()
        .filter(
            AppVersionColumn::Country
                .eq(country)
                .and(AppVersionColumn::AppId.eq(app_id)),
        )
        .order_by_desc(AppVersionColumn::CreatedAt)
        .all(conn)
        .await
}

#[instrument(skip(conn))]
pub async fn latest_version_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Option<AppVersionModel>, DbErr> {
    AppVersion::find()
        .filter(
            AppVersionColumn::Country
                .eq(country)
                .and(AppVersionColumn::AppId.eq(app_id)),
        )
        .order_by_desc(AppVersionColumn::CreatedAt)
        .limit(1)
        .one(conn)
        .await
}

#[cfg(test)]
mod tests {
    use entity::app::AppCountry;
    use sea_orm::{ConnectOptions, Database, Value};
    use std::env;
    use std::time::Duration;
    use tracing::{debug, info, log};

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
        let app_ids = vec!["1".to_owned(), "2".to_owned()];
        // let s: Value = app_ids.into();
        // debug!("sql: {}",s)
        let lists = super::search_by_appids(&conn, AppCountry::Cn, app_ids)
            .await
            .unwrap();
        assert_eq!(lists.len(), 1);
    }
}
