use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;
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

#[derive(FromQueryResult)]
pub struct NewModel {
    pub app_id: String,
    pub country: AppCountry,
    pub version: String,
    pub size: i64,
}

pub struct VersionInfo {
    pub country: AppCountry,
    pub app_id: String,
    pub version: String,
    pub size: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
