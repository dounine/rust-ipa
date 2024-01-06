use ::entity::user::Model;
use ::entity::user::Entity;
use ::entity::user::ActiveModel;
use ::entity::user;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn create_user(
    conn: &DbConn,
    form_data: Model,
) -> Result<ActiveModel, DbErr> {
    ActiveModel {
        nick_name: Set(form_data.nick_name.to_owned()),
        email: Set(form_data.email.to_owned()),
        password: Set(form_data.password.to_owned()),
        ..Default::default()
    }
        .save(conn)
        .await
}

#[instrument(skip(conn))]
pub async fn list_user(
    conn: &DbConn,
    offset: u64,
    limit: u64,
) -> Result<(Vec<Model>, u64), DbErr> {
    let paginator = Entity::find()
        .order_by_desc(user::Column::Id)
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}

#[instrument(skip(db))]
pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(db).await
}