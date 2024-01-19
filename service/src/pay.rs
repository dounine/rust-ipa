use crate::error::ServiceError;
use ::entity::pay::PayPlatform;
use ::entity::{Pay, PayActiveModel, PayColumn};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DbConn, EntityTrait, TransactionTrait};
use tracing::instrument;

/// 创建订单
#[instrument(skip(conn))]
pub async fn create_pay(
    conn: &DbConn,
    user_id: i32,
    platform: PayPlatform,
    money: i32,
    coin: i32,
) -> Result<i32, ServiceError> {
    let pay = PayActiveModel {
        user_id: Set(user_id),
        money: Set(money),
        coin: Set(coin),
        platform: Set(platform),
        payed: Set(false),
        ..Default::default()
    };
    let pay_id = Pay::insert(pay).exec(conn).await?.last_insert_id;
    Ok(pay_id)
}

/// 修改订单状态
#[instrument(skip(conn))]
pub async fn change_payed_status(
    conn: &DbConn,
    pay_id: i32,
    user_id: i32,
) -> Result<(), ServiceError> {
    let tx = conn.begin().await?;
    let pay_info = Pay::find()
        .filter(PayColumn::Id.eq(pay_id))
        .one(&tx)
        .await?;
    match pay_info {
        Some(info) => {
            if info.payed {
                return Err(ServiceError::Msg("订单已支付，请不要重复支付".to_string()));
            }
            let coin = info.coin;
            let mut acive_model: PayActiveModel = info.into();
            acive_model.payed = Set(true);
            Pay::update(acive_model).exec(&tx).await?;
            super::pay_record::user_coin_change(
                &tx,
                user_id,
                coin,
                entity::pay_record::PayRecordType::Charge,
            )
            .await?;
            tx.commit().await?;
        }
        None => return Err(ServiceError::Msg("订单不存在".to_string())),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;
    use tracing::debug;

    #[tokio::test]
    async fn test_change_payed_status() {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::init();
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let conn = Database::connect(db_url)
            .await
            .expect("Cannot connect to database");
        let pay_id = super::create_pay(&conn, 1, super::PayPlatform::Wechat, 1, 1)
            .await
            .unwrap();
        debug!("pay_id: {}", pay_id);
        super::change_payed_status(&conn, pay_id, 1).await.unwrap();
    }
}
