use sea_orm::entity::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserStatus {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Disable,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserType {
    #[sea_orm(num_value = 0)]
    User,
    #[sea_orm(num_value = 1)]
    Admin,
    #[sea_orm(num_value = 2)]
    Guest,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserPlatform {
    #[sea_orm(num_value = 0)]
    Email,
    #[sea_orm(num_value = 1)]
    Wechat,
    #[sea_orm(num_value = 2)]
    QQ,
    #[sea_orm(num_value = 3)]
    Username,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
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
    pub avatar: Option<String>,
    pub status: UserStatus,
    pub platform: UserPlatform,
    pub user_type: UserType,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}