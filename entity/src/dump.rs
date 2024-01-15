use crate::app::AppCountry;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum DumpStatus {
    //等待中
    Waiting = 0,
    //提取中
    Dumping = 1,
    //提取完成
    Done = 2,
    //不可提取
    UnDump = 3,
    //越狱检测
    Check = 4,
    //暂停
    Pause = 5,
    //版本过旧
    Old = 6,
    //付费应用
    Pay = 7,
    //下架应用
    Off = 8,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
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
