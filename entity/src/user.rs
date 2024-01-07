use sea_orm::entity::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserStatus {
    Normal = 0,
    Disable = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserType {
    User = 0,
    Admin = 1,
    Guest = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserPlatform {
    Email = 0,
    Wechat = 1,
    QQ = 2,
    Username = 3,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sea_orm(indexed)]
    pub user_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sea_orm(indexed)]
    pub email: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: Option<String>,
    pub channel_code: String,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sea_orm(column_type = "Text")]
    pub avatar: Option<String>,
    pub status: UserStatus,
    pub platform: UserPlatform,
    pub user_type: UserType,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}