use sea_orm_migration::prelude::*;

use entity::{AppVersion, AppVersionActiveModel};
use entity::app::AppCountry;

use crate::sea_orm::{ActiveModelTrait, EntityName, Set, TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppVersion.table_ref())
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk-app_version")
                            .col(AppVersions::AppId)
                            .col(AppVersions::Country)
                            .col(AppVersions::Version)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(AppVersions::AppId)
                            .string_len(20)
                            .not_null()
                            .comment("应用ID"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Country)
                            .string_len(10)
                            .not_null()
                            .comment("地区"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Version)
                            .string_len(20)
                            .not_null()
                            .comment("版本号"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Des)
                            .string_len(200)
                            .not_null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Download)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("下载次数"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Official)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("是否官方"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::DownloadUrl)
                            .text()
                            .not_null()
                            .comment("下载地址"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::Size)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("应用大小"),
                    )
                    .col(
                        ColumnDef::new(AppVersions::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(AppVersion.table_ref())
                    .name("idx-app-created_at")
                    .col(AppVersions::CreatedAt)
                    .to_owned(),
            ).await?;
        //AppId+Country 联合索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(AppVersion.table_ref())
                    .name("idx-app-app_id_country")
                    .col(AppVersions::AppId)
                    .col(AppVersions::Country)
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        AppVersionActiveModel {
            app_id: Set("1".to_owned()),
            country: Set(AppCountry::Cn),
            version: Set("1.0.0".to_owned()),
            des: Set("测试".to_owned()),
            download: Set(0),
            official: Set(true),
            download_url: Set("https://www.baidu.com".to_owned()),
            size: Set(1024 * 1024),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().table(AppVersion.table_ref()).name("idx-app-created_at").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().table(AppVersion.table_ref()).name("idx-app-app_id_country").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(AppVersion.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AppVersions {
    AppId,
    Country,
    Version,
    Des,
    Download,
    Size,
    Official,
    DownloadUrl,
    CreatedAt,
}