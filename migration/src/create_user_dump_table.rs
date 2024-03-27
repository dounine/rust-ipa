use entity::app::AppCountry;
use entity::{UserDump, UserDumpActiveModel};
use sea_orm_migration::prelude::*;

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
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(UserDump.table_ref())
                    .name("idx-user_dump-user_id")
                    .col(UserDumps::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(UserDump.table_ref())
                    .name("idx-user_dump-created_at")
                    .col(UserDumps::CreatedAt)
                    .to_owned(),
            )
            .await?;
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
        // tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .table(UserDump.table_ref())
                    .name("idx-user_dump-user_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .table(UserDump.table_ref())
                    .name("idx-user_dump-created_at")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(UserDump.table_ref())
                    .to_owned(),
            )
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
