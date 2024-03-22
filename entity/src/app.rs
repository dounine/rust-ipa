use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(Some(10))")]
pub enum AppCountry {
    #[sea_orm(string_value = "cn")]
    Cn,
    #[sea_orm(string_value = "us")]
    Us,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum AppPlatform {
    #[sea_orm(num_value = 0)]
    Signer = 0,
    // 自签ipa
    #[sea_orm(num_value = 1)]
    TrollStore = 1,
    //巨魔
    #[sea_orm(num_value = 2)]
    Cydia = 2, //越狱
    #[sea_orm(num_value = 3)]
    All = 3,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Eq)]
#[sea_orm(table_name = "app")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub app_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub country: AppCountry,
    pub name: String,
    pub origin_name: String,
    pub bundle_id: String,
    pub des: String,
    #[sea_orm(column_type = "Text")]
    pub icon: String,
    pub platform: AppPlatform,
    pub price: i32,
    pub genres: String,
    pub single: bool,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
