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
) -> Result<String, ServiceError> {
    let pay = PayActiveModel {
        id: Set(util::uuid::uuid32()),
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
    pay_id: String,
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
            let mut acive_model: PayActiveModel = info.clone().into();
            acive_model.payed = Set(true);
            acive_model.payed_time = Set(Some(util::time::now()));
            Pay::update(acive_model).exec(&tx).await?;
            super::pay_record::user_coin_change(
                &tx,
                info.user_id,
                info.coin,
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
    use crate::error::ServiceError;
    use sea_orm::Database;
    use tracing::debug;

    /// 测试创建订单
    #[tokio::test]
    async fn test_change_payed_status() -> Result<(), ServiceError> {
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
        super::change_payed_status(&conn, pay_id).await
    }

    /// 测试订单不存在
    #[tokio::test]
    #[should_panic(expected = "订单不存在")]
    async fn test_change_payed_status_not_found() {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::init();
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let conn = Database::connect(db_url)
            .await
            .expect("Cannot connect to database");
        super::change_payed_status(&conn, "-1".to_string())
            .await
            .unwrap();
    }

    /// 测试重复支付
    #[tokio::test]
    #[should_panic(expected = "订单已支付，请不要重复支付")]
    async fn test_change_payed_status_repeat_pay() {
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
        super::change_payed_status(&conn, pay_id.clone())
            .await
            .unwrap();
        super::change_payed_status(&conn, pay_id).await.unwrap();
    }
}
