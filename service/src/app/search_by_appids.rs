use std::fmt::Debug;

use ::entity::app::{AppCountry};
use ::entity::App;
use ::entity::AppColumn;
use ::entity::AppModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn search_by_appids<S>(
    conn: &DbConn,
    country: &AppCountry,
    app_ids: Vec<S>,
) -> Result<Vec<AppModel>, DbErr>
where
    S: AsRef<str> + Debug,
{
    let app_ids = app_ids.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
    App::find()
        .filter(
            AppColumn::Country
                .eq(country.clone())
                .and(AppColumn::AppId.is_in(app_ids)),
        )
        .all(conn)
        .await
}