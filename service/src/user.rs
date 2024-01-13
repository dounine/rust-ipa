use ::entity::User;
use ::entity::UserActiveModel;
use ::entity::UserColumn;
use ::entity::UserModel;
use sea_orm::*;
use tracing::instrument;

#[instrument(skip(conn))]
pub async fn create_user(conn: &DbConn, form_data: UserModel) -> Result<UserActiveModel, DbErr> {
    UserActiveModel {
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
) -> Result<(Vec<UserModel>, u64), DbErr> {
    let paginator = User::find()
        .order_by_desc(UserColumn::Id)
        .paginate(conn, limit);
    let num_pages = paginator.num_pages().await?;
    paginator
        .fetch_page(offset)
        .await
        .map(|list| (list, num_pages))
}

#[instrument(skip(conn))]
pub async fn find_user_by_id(conn: &DbConn, id: i32) -> Result<Option<UserModel>, DbErr> {
    User::find_by_id(id).one(conn).await
}

#[instrument(skip(conn))]
pub async fn find_user_by_email(conn: &DbConn, email: &str) -> Result<Option<UserModel>, DbErr> {
    User::find()
        .filter(UserColumn::Email.eq(email))
        .one(conn)
        .await
}
#[instrument(skip(conn))]
pub async fn find_user_by_username(
    conn: &DbConn,
    user_name: &str,
) -> Result<Option<UserModel>, DbErr> {
    User::find()
        .filter(UserColumn::UserName.eq(user_name))
        .one(conn)
        .await
}
