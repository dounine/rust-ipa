use sea_orm_migration::prelude::*;

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
                    .to_owned(),
            ).await
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
    Email,
    Password,
}