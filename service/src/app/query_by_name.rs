use std::fmt::Debug;

use ::entity::app::{AppCountry};
use ::entity::App;
use ::entity::AppColumn;
use ::entity::AppModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn query_by_name<S>(
    conn: &DbConn,
    country: &AppCountry,
    name: S,
) -> Result<Vec<AppModel>, DbErr>
where
    S: AsRef<str> + Debug,
{
    let name = name.as_ref();
    App::find()
        .filter(
            AppColumn::Country.eq(country.clone()).and(
                AppColumn::Name
                    .eq(name)
                    .or(AppColumn::Name.contains(name))
                    .or(AppColumn::AppId.eq(name)),
            ),
        )
        .limit(3)
        .all(conn)
        .await
}
