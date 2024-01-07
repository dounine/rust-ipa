use sea_orm_migration::prelude::*;

use entity::{Pay, PayActiveModel, User, UserActiveModel};
use entity::pay::PayPlatform;

use crate::sea_orm::{ActiveModelTrait, EntityName, Set, TransactionTrait};
use crate::sea_orm::prelude::DateTime;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pay.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pays::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("支付ID"),
                    )
                    .col(
                        ColumnDef::new(Pays::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Pays::Money)
                            .integer()
                            .not_null()
                            .comment("支付金额"),
                    )
                    .col(
                        ColumnDef::new(Pays::Coin)
                            .integer()
                            .not_null()
                            .comment("支付金币"),
                    )
                    .col(
                        ColumnDef::new(Pays::TradeNo)
                            .string_len(100)
                            .null()
                            .comment("支付订单号"),
                    )
                    .col(
                        ColumnDef::new(Pays::Platform)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("支付平台/0:微信,1:支付宝"),
                    )
                    .col(
                        ColumnDef::new(Pays::PayedTime)
                            .date_time()
                            .null()
                            .comment("支付时间"),
                    )
                    .col(
                        ColumnDef::new(Pays::Payed)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否支付"),
                    )
                    .col(
                        ColumnDef::new(Pays::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        PayActiveModel {
            id: Set(1),
            user_id: Set(1),
            money: Set(800),
            coin: Set(800),
            payed: Set(true),
            platform: Set(PayPlatform::Weixin),
            payed_time: Set(Some(DateTime::default())),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pay.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pays {
    Id,
    UserId,
    Money,
    Coin,
    TradeNo,
    Payed,
    Platform,
    PayedTime,
    CreatedAt,
}