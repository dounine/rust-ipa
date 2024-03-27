use sea_orm_migration::prelude::*;

use entity::app::{AppCountry, AppPlatform};
use entity::{App, AppActiveModel};

use crate::sea_orm::{ActiveModelTrait, EntityName, Set, TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(App.table_ref())
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk-app")
                            .col(Apps::AppId)
                            .col(Apps::Country)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(Apps::AppId)
                            .string_len(20)
                            .not_null()
                            .comment("应用ID"),
                    )
                    .col(
                        ColumnDef::new(Apps::Country)
                            .string_len(10)
                            .not_null()
                            .comment("地区"),
                    )
                    .col(
                        ColumnDef::new(Apps::Name)
                            .string_len(100)
                            .not_null()
                            .comment("应用名称"),
                    )
                    .col(
                        ColumnDef::new(Apps::OriginName)
                            .string_len(100)
                            .not_null()
                            .comment("原始名称"),
                    )
                    .col(
                        ColumnDef::new(Apps::BundleId)
                            .string_len(100)
                            .not_null()
                            .comment("包名"),
                    )
                    .col(
                        ColumnDef::new(Apps::Des)
                            .string_len(200)
                            .not_null()
                            .comment("描述"),
                    )
                    .col(ColumnDef::new(Apps::Icon).text().not_null().comment("图标"))
                    .col(
                        ColumnDef::new(Apps::Platform)
                            .tiny_integer()
                            .not_null()
                            .comment("平台/0:自签/1:巨魔/2:Cydia"),
                    )
                    .col(
                        ColumnDef::new(Apps::Price)
                            .integer()
                            .not_null()
                            .comment("价格"),
                    )
                    .col(
                        ColumnDef::new(Apps::Genres)
                            .string_len(100)
                            .not_null()
                            .comment("类型"),
                    )
                    .col(
                        ColumnDef::new(Apps::Single)
                            .boolean()
                            .not_null()
                            .comment("是否单包"),
                    )
                    .col(
                        ColumnDef::new(Apps::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-app-name")
                    .table(App.table_ref())
                    .col(Apps::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-app-app_id")
                    .table(App.table_ref())
                    .col(Apps::AppId)
                    .to_owned(),
            )
            .await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
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
            .insert(conn)
            .await?;
        // tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-app-name")
                    .table(App.table_ref())
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-app-app_id")
                    .table(App.table_ref())
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(App.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Apps {
    AppId,
    Country,
    Name,
    OriginName,
    BundleId,
    Des,
    Icon,
    Platform,
    Price,
    Genres,
    Single,
    CreatedAt,
}
