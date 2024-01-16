use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum PayRecordType {
    //充值
    Charge = 0,
    //提取
    Extract = 1,
    //下载
    Download = 2,
    //赠送
    Give = 3,
    //收到
    Receive = 4,
    //退款
    Refund = 5,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Eq)]
#[sea_orm(table_name = "pay_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub coin: i32,
    pub record_type: PayRecordType,
    pub des: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
