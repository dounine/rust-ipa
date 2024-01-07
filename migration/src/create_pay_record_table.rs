use sea_orm_migration::prelude::*;

use entity::{PayRecord, PayRecordActiveModel};
use entity::pay_record::PayRecordType;

use crate::sea_orm::{ActiveModelTrait, EntityName, Set, TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PayRecord.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("支付记录ID"),
                    )
                    .col(
                        ColumnDef::new(PayRecords::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PayRecords::Coin)
                            .integer()
                            .not_null()
                            .comment("支付金币"),
                    )
                    .col(
                        ColumnDef::new(PayRecords::RecordType)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("支付类型/0:充值,1:提取,2:下载,3:赠送,4:收到,5:退款"),
                    )
                    .col(
                        ColumnDef::new(PayRecords::Des)
                            .string_len(100)
                            .null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(PayRecords::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        PayRecordActiveModel {
            id: Set(1),
            user_id: Set(1),
            coin: Set(800),
            record_type: Set(PayRecordType::Charge),
            des: Set(Some("充值".to_owned())),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PayRecord.table_ref()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PayRecords {
    Id,
    UserId,
    Coin,
    RecordType,
    Des,
    CreatedAt,
}