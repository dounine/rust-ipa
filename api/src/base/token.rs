use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use entity::user::UserType;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use migration::DbErr;
use serde::{Deserialize, Serialize};
use service::sea_orm::DbConn;
use std::future::Future;
use std::pin::Pin;
use crate::base::error::MyError;
use crate::base::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub user_type: UserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user_type: UserType,
    pub id: i32,
}

static JWT_SECRET: &'static str = "secret";

fn with_exp(seconds: i64) -> usize {
    let exp = chrono::Local::now() + chrono::Duration::seconds(seconds);
    exp.timestamp() as usize
}

pub async fn validate_token(token: &str, conn: &DbConn) -> Result<Option<UserData>, MyError> {
    let data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| MyError::TokenError(e));
    match data {
        Ok(data) => match data.user_type {
            UserType::Admin | UserType::User => service::user::find_user_by_id(&conn, data.id)
                .await
                .map(|user| {
                    user.map(|user| UserData {
                        id: user.id,
                        user_type: user.user_type,
                    })
                })
                .map_err(|e| MyError::DbError(e)),
            UserType::Guest => Ok(Some(UserData {
                id: 0,
                user_type: UserType::Guest,
            })),
        },
        Err(e) => Err(e),
    }
}

pub fn create_token(user_id: i32, user_type: UserType, exp: i64) -> Result<String, String> {
    let claim = Claims {
        id: user_id,
        user_type,
        exp: with_exp(exp),
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|e| e.to_string())
}

impl FromRequest for UserData {
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data = req
                .headers()
                .get("Authorization")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.split("Bearer ").last());
            match data {
                Some(token) => {
                    let state = req.app_data::<Data<AppState>>().unwrap();
                    validate_token(token, &state.conn)
                        .await
                        .and_then(|user| user.ok_or(MyError::Msg("用户不存在".to_string())))
                }
                None => Err(MyError::Msg("invalid token".to_string())),
            }
        })
    }
}
