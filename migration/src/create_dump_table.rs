use sea_orm_migration::prelude::*;

use entity::{Dump, DumpActiveModel};
use entity::app::AppCountry;
use entity::dump::DumpStatus;

use crate::sea_orm::{ActiveModelTrait, EntityName, Set, TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dump.table_ref())
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk-dump")
                            .col(Dumps::AppId)
                            .col(Dumps::Country)
                            .col(Dumps::Version)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(Dumps::AppId)
                            .string_len(20)
                            .not_null()
                            .comment("应用ID"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Country)
                            .string_len(10)
                            .not_null()
                            .comment("地区"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Version)
                            .string_len(20)
                            .not_null()
                            .comment("版本号"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Name)
                            .string_len(100)
                            .not_null()
                            .comment("应用名称"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Icon)
                            .text()
                            .not_null()
                            .comment("应用图标"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Link)
                            .text()
                            .not_null()
                            .comment("应用链接"),
                    )
                    .col(
                        ColumnDef::new(Dumps::BundleId)
                            .string_len(100)
                            .not_null()
                            .comment("应用包名"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Size)
                            .big_integer()
                            .not_null()
                            .comment("应用大小"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Price)
                            .integer()
                            .not_null()
                            .comment("应用价格"),
                    )
                    .col(
                        ColumnDef::new(Dumps::Status)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("应用状态/0:等待中,1:提取中,2:提取完成,3:不可提取,4:越狱检测,5:暂停,6:版本过旧,7:付费应用,8:下架"),
                    )
                    .col(
                        ColumnDef::new(Dumps::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        DumpActiveModel {
            app_id: Set("1".to_owned()),
            country: Set(AppCountry::Cn),
            version: Set("1.0.0".to_owned()),
            name: Set("微信".to_owned()),
            icon: Set("https://www.baidu.com".to_owned()),
            link: Set("https://www.baidu.com".to_owned()),
            bundle_id: Set("com.tencent.xin".to_owned()),
            size: Set(1024 * 1024),
            price: Set(0),
            status: Set(DumpStatus::Waiting),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dump.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Dumps {
    AppId,
    Country,
    Version,
    Name,
    Icon,
    Link,
    BundleId,
    Size,
    Price,
    Status,
    CreatedAt,
}