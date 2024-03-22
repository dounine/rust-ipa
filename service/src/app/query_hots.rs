use std::fmt::Debug;

use ::entity::app::AppCountry;
use sea_orm::*;
use tracing::instrument;
use serde::Serialize;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct AppHot {
    app_id: String,
    country: AppCountry,
    version: String,
    name: String,
    des: String,
    icon: String,
    price: i32,
    genres: String,
    size: i64,
}

#[instrument(skip(conn))]
pub async fn query_hots(conn: &DbConn, offset: u64, limit: u64) -> Result<Vec<AppHot>, DbErr> {
    AppHot::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
            SELECT cc.name as name,
       dd.app_id,
       cc.des,
       cc.platform,
       cc.icon,
       cc.price,
       cc.genres,
       cc.country,
       dd.version,
       dd.size
FROM (SELECT app_id,
             country,
             SUM(a.download) as download,
             (SELECT c.version
              from app_version as c
              where c.app_id = a.app_id
              order by c.created_at desc
              limit 1)       as version,
             (SELECT c.size
              from app_version as c
              where c.app_id = a.app_id
              order by c.created_at desc
              limit 1)       as size,
             (SELECT c.official
              from app_version as c
              where c.app_id = a.app_id
              order by c.created_at desc
              limit 1)       as official
      FROM app_version as a
      GROUP BY app_id, country
      ORDER BY download DESC
      OFFSET 0 LIMIT 9) dd
         INNER JOIN app cc ON dd.app_id = cc.app_id AND dd.country = cc.country;"#,
        vec![],
    ))
    .all(conn)
    .await
}
