use std::fmt::Debug;

use ::entity::app::AppCountry;
use sea_orm::*;
use serde::Serialize;
use tracing::instrument;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct AppItem {
    pub app_id: String,
    pub country: AppCountry,
    pub version: String,
    pub name: String,
    pub icon: String,
    pub price: i32,
    pub genres: String,
    #[serde(serialize_with = "format_file_size")]
    pub size: i64,
}

fn format_file_size<S>(size: &i64, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let format_str = util::file::byte_format(*size);
    s.serialize_str(&format_str)
}

#[instrument(skip(conn))]
pub async fn query_hots(conn: &DbConn, offset: u64, limit: u64) -> Result<Vec<AppItem>, DbErr> {
    AppItem::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
            SELECT cc.name as name,
       dd.app_id,
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
      OFFSET $1 LIMIT $2) dd
         INNER JOIN app cc ON dd.app_id = cc.app_id AND dd.country = cc.country"#,
        vec![offset.into(), limit.into()],
    ))
    .all(conn)
    .await
}
