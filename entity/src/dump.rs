use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::app::AppCountry;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum DumpStatus {
    Waiting = 0,
    //等待中
    Dumping = 1,
    //提取中
    Done = 2,
    //提取完成
    UnDump = 3,
    //不可提取
    Check = 4,
    //越狱检测
    Pause = 5,
    //暂停
    Old = 6,
    //版本过旧
    Pay = 7,//付费应用
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "dump")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub app_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub country: AppCountry,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub icon: String,
    #[sea_orm(column_type = "Text")]
    pub link: String,
    pub bundle_id: String,
    pub size: i64,
    pub price: i32,
    pub status: DumpStatus,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
