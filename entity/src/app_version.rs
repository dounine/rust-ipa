use sea_orm::entity::prelude::*;
use crate::app::AppCountry;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "app_version")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub app_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub country: AppCountry,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    pub des: String,
    pub download: i32,
    pub size: i64,
    pub official: bool,
    #[sea_orm(column_type = "Text")]
    pub download_url: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
