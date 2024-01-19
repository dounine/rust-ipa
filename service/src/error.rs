#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    Msg(String),
    #[error("db_error: {0}")]
    DbError(#[from] sea_orm::DbErr),
}
