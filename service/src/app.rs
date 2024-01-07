use ::entity::app::{AppCountry, AppPlatform};
use ::entity::AppColumn;
use ::entity::AppModel;
use ::entity::AppActiveModel;
use ::entity::App;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn create(
    conn: &DbConn,
    form_data: AppModel,
) -> Result<(), DbErr> {
    AppActiveModel {
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
    }
        .save(conn)
        .await
        .map(|_| ())
}

#[instrument(skip(conn))]
pub async fn list(
    conn: &DbConn,
    offset: u64,
    limit: u64,
) -> Result<(Vec<AppModel>, u64), DbErr> {
    let paginator = App::find()
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}