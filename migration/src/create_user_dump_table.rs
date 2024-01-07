use sea_orm_migration::prelude::*;
use entity::{UserDump, UserDumpActiveModel};
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
                    .table(UserDump.table_ref())
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk-user_dump")
                            .col(UserDumps::UserId)
                            .col(UserDumps::AppId)
                            .col(UserDumps::Country)
                            .col(UserDumps::Version)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(UserDumps::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserDumps::AppId)
                            .string_len(20)
                            .not_null()
                            .comment("应用ID"),
                    )
                    .col(
                        ColumnDef::new(UserDumps::Country)
                            .string_len(10)
                            .not_null()
                            .comment("地区"),
                    )
                    .col(
                        ColumnDef::new(UserDumps::Version)
                            .string_len(20)
                            .not_null()
                            .comment("版本号"),
                    )
                    .col(
                        ColumnDef::new(UserDumps::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        UserDumpActiveModel {
            user_id: Set(1),
            app_id: Set("1".to_owned()),
            country: Set(AppCountry::Cn),
            version: Set("1.0.0".to_owned()),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserDump.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserDumps {
    UserId,
    AppId,
    Country,
    Version,
    CreatedAt,
}