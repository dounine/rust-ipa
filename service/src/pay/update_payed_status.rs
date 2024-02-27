use sea_orm::{ColumnTrait};
use sea_orm::{DbConn, EntityTrait, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use tracing::instrument;

use ::entity::{Pay, PayActiveModel, PayColumn};
use entity::pay_record::PayRecordType;

use crate::error::ServiceError;

#[instrument(skip(conn))]
pub async fn update_payed_status(conn: &DbConn, pay_id: String) -> Result<(), ServiceError> {
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
            crate::pay_record::update_coin::update_coin(
                &tx,
                info.user_id,
                info.coin,
                PayRecordType::Charge,
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

    use crate::error::ServiceError;
    use crate::pay::add::add;
    use crate::pay::update_payed_status::update_payed_status;

    /// 测试创建订单
    #[tokio::test]
    async fn test_change_payed_status() -> Result<(), ServiceError> {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::init();
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let conn = Database::connect(db_url)
            .await
            .expect("Cannot connect to database");
        let pay_info = add(&conn, 1, super::PayPlatform::Wechat, 1, 1).await?;
        debug!("pay_id: {:?}", pay_info);
        let pay_id = pay_info.id.unwrap();
        update_payed_status(&conn, pay_id).await
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
        update_payed_status(&conn, "-1".to_string())
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
        let pay_info = add(&conn, 1, super::PayPlatform::Wechat, 1, 1)
            .await
            .unwrap();
        debug!("pay_info: {:?}", pay_info);
        let pay_id = pay_info.id.unwrap();
        update_payed_status(&conn, pay_id.clone())
            .await
            .unwrap();
        update_payed_status(&conn, pay_id).await.unwrap();
    }
}

