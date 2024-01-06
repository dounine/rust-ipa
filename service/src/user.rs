use ::entity::user::Entity;
use ::entity::user::ActiveModel;
use ::entity::user;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(db))]
pub async fn create_user(
    db: &DbConn,
    form_data: user::Model,
) -> Result<ActiveModel, DbErr> {
    ActiveModel {
        nick_name: Set(form_data.nick_name.to_owned()),
        email: Set(form_data.email.to_owned()),
        password: Set(form_data.password.to_owned()),
        ..Default::default()
    }
        .save(db)
        .await
}

#[instrument(skip(db))]
pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
    Entity::find_by_id(id).one(db).await
}