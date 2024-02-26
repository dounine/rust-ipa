use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DbConn;
use sea_orm::DbErr;
use tracing::instrument;

use ::entity::pay::PayPlatform;
use ::entity::PayActiveModel;
use ::entity::PayModel;

// 创建订单
#[instrument(skip(conn))]
pub async fn create(
    conn: &DbConn,
    user_id: i32,
    platform: PayPlatform,
    money: i32,
    coin: i32,
) -> Result<PayModel, DbErr> {
    let model = PayActiveModel {
        id: Set(util::uuid::uuid32()),
        user_id: Set(user_id),
        money: Set(money),
        coin: Set(coin),
        platform: Set(platform),
        payed: Set(false),
        ..Default::default()
    };
    model.insert(conn).await
}