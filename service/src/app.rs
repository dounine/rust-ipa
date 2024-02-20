use ::entity::app::{AppCountry, AppPlatform};
use ::entity::App;
use ::entity::AppActiveModel;
use ::entity::AppColumn;
use ::entity::AppModel;
use sea_orm::*;
use std::fmt::Debug;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn create(conn: &DbConn, form_data: AppModel) -> Result<(), DbErr> {
    let model = AppActiveModel {
        app_id: Set("1".to_owned()),
        country: Set(AppCountry::Cn),
        name: Set("微信".to_owned()),
        origin_name: Set("腾讯微信".to_owned()),
        bundle_id: Set("com.tencent.xin".to_owned()),
        des: Set("微信是一款跨平台的通讯工具。支持单人、多人参与。通过手机网络发送语音、图片、视频和文字。".to_owned()),
        icon: Set("https://is4-ssl.mzstatic.com/image/thumb/Purple123/v4/0b/f9/6e/0bf96e4f-75e1-40db-d02e-d32a8fb6475a/AppIcon-0-1x_U007emarketing-0-4-0-sRGB-0-85-220.png/512x512bb.jpg".to_owned()),
        platform: Set(AppPlatform::Signer),
        price: Set(0),
        genres: Set("社交".to_owned()),
        single: Set(false),
        ..Default::default()
    };
    model.insert(conn).await.map(|_| ())
}

#[instrument(skip(conn))]
pub async fn search_by_appid(
    conn: &DbConn,
    country: AppCountry,
    app_id: &str,
) -> Result<Option<AppModel>, DbErr> {
    App::find()
        .filter(
            AppColumn::Country
                .eq(country)
                .and(AppColumn::AppId.eq(app_id)),
        )
        .one(conn)
        .await
}

#[instrument(skip(conn))]
pub async fn search_by_name<S>(
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

#[instrument(skip(conn))]
pub async fn list(conn: &DbConn, offset: u64, limit: u64) -> Result<(Vec<AppModel>, u64), DbErr> {
    let paginator = App::find().paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}
