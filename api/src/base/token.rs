use crate::base::error::ApiError;
use crate::base::state::AppState;
use actix_web::dev::Payload;
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use entity::user::UserType;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use service::sea_orm::DbConn;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub user_type: UserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserData {
    pub id: i32,
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

pub async fn validate_token(token: &str, conn: &DbConn) -> Result<Option<UserData>, ApiError> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map(|claims| async move {
        match claims.user_type {
            UserType::Admin | UserType::User => service::user::find_user_by_id(&conn, claims.id)
                .await
                .map(|user| {
                    user.map(|user| UserData {
                        id: user.id,
                        user_type: user.user_type,
                    })
                })
                .map_err(|e| ApiError::DbError(e)),
            UserType::Guest => Ok(Some(UserData {
                id: 0,
                user_type: UserType::Guest,
            })),
        }
    })?
    .await
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
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            if true {
                return Ok(UserData {
                    id: 1,
                    user_type: UserType::Admin,
                });
            }
            let state = req.app_data::<Data<AppState>>().unwrap();
            req.headers()
                .get(AUTHORIZATION)
                .and_then(|header_value| header_value.to_str().ok())
                .and_then(|authorization| authorization.split("Bearer ").last())
                .map(|token| async move {
                    validate_token(token, &state.conn)
                        .await
                        .and_then(|user| user.ok_or(ApiError::msg("用户不存在")))
                })
                .ok_or_else(|| ApiError::msg("missing token"))?
                .await
        })
    }
}

impl FromRequest for AdminUserData {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            if true {
                return Ok(AdminUserData { id: 1 });
            }
            let state = req.app_data::<Data<AppState>>().unwrap();
            req.headers()
                .get(AUTHORIZATION)
                .and_then(|header_value| header_value.to_str().ok())
                .and_then(|authorization| authorization.split("Bearer ").last())
                .map(|token| async move {
                    validate_token(token, &state.conn)
                        .await
                        .map(|user_opt| user_opt.map(|user| AdminUserData { id: user.id }))
                        .and_then(|user| user.ok_or(ApiError::msg("用户不存在")))
                })
                .ok_or_else(|| ApiError::msg("missing token"))?
                .await
        })
    }
}
