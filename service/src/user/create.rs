use sea_orm::*;
use tracing::instrument;

use ::entity::UserActiveModel;
use ::entity::UserModel;

#[instrument(skip(conn))]
pub async fn create(conn: &DbConn, form_data: UserModel) -> Result<UserModel, DbErr> {
    let model = UserActiveModel {
        nick_name: Set(form_data.nick_name.to_owned()),
        email: Set(form_data.email.to_owned()),
        password: Set(form_data.password.to_owned()),
        ..Default::default()
    };
    model.insert(conn).await
}
