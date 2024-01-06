use sea_orm_migration::prelude::*;
use crate::sea_orm::{ActiveModelTrait, NotSet, Set, TransactionTrait};
use ::entity::user::ActiveModel;
use entity::user::{UserPlatform, UserStatus, UserType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Users::NickName)
                            .string_len(50)
                            .null()
                            .comment("用户昵称"),
                    )
                    .col(
                        ColumnDef::new(Users::UserName)
                            .string_len(50)
                            .null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(50)
                            .null()
                            .comment("用户邮箱"),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string_len(32)
                            .null()
                            .comment("用户密码"),
                    )
                    .col(
                        ColumnDef::new(Users::ChannelCode)
                            .string_len(50)
                            .not_null()
                            .comment("来源渠道"),
                    )
                    .col(
                        ColumnDef::new(Users::Ip)
                            .string_len(15)
                            .null()
                            .comment("用户IP"),
                    )
                    .col(
                        ColumnDef::new(Users::Uid)
                            .string_len(50)
                            .null()
                            .comment("用户唯一标识"),
                    )
                    .col(
                        ColumnDef::new(Users::Avatar)
                            .text()
                            .null()
                            .comment("用户头像"),
                    )
                    .col(
                        ColumnDef::new(Users::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("用户状态"),
                    )
                    .col(
                        ColumnDef::new(Users::Platform)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("用户平台"),
                    )
                    .col(
                        ColumnDef::new(Users::UserType)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("用户类型"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .comment("创建时间"),
                    )
                    .to_owned(),
            ).await?;
        let conn = manager.get_connection();
        let tx = conn.begin().await?;
        ActiveModel {
            id: Set(1),
            nick_name: Set(Some("lake".to_owned())),
            user_name: Set(Some("lake".to_owned())),
            channel_code: Set("ipa.com".to_owned()),
            ip: Set("127.0.0.1".to_owned()),
            status: Set(UserStatus::Normal),
            platform: Set(UserPlatform::Email),
            user_type: Set(UserType::User),
            ..Default::default()
        }
            .insert(conn)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    NickName,
    UserName,
    Email,
    Password,
    ChannelCode,
    Ip,
    Uid,
    Avatar,
    Status,
    Platform,
    UserType,
    CreatedAt,
}