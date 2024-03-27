use entity::{PayMenu, PayMenuActiveModel};
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
                    .table(PayMenu.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayMenus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(PayMenus::Money)
                            .integer()
                            .not_null()
                            .comment("金额分"),
                    )
                    .col(
                        ColumnDef::new(PayMenus::Coin)
                            .integer()
                            .not_null()
                            .comment("充值金币分"),
                    )
                    .col(
                        ColumnDef::new(PayMenus::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        PayMenuActiveModel {
            id: Set(1.to_owned()),
            money: Set(800.to_owned()),
            coin: Set(800.to_owned()),
            ..Default::default()
        }
        .insert(conn)
        .await?;
        // tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PayMenu.table_ref())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PayMenus {
    Id,
    Money,
    Coin,
    CreatedAt,
}
