use std::future::{Future};
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use entity::user::UserType;
use service::sea_orm::DbConn;
use crate::error::MyError;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub user_type: UserType,
}

static JWT_SECRET: &'static str = "secret";

pub async fn validate_token(token: &str, conn: &DbConn) -> Result<Option<UserData>, String> {
    let data = decode::<UserData>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| e.to_string());
    match data {
        Ok(data) => {
            match data.user_type {
                UserType::Admin | UserType::User => {
                    service::user::find_user_by_id(&conn, data.id)
                        .await
                        .map(|user| user.map(|user| UserData {
                            id: user.id,
                            user_type: user.user_type,
                        }))
                        .map_err(|e| e.to_string())
                }
                UserType::Guest => {
                    Ok(Some(UserData {
                        id: 0,
                        user_type: UserType::Guest,
                    }))
                }
            }
        }
        Err(e) => Err(e),
    }
}

pub fn create_token(user: &UserData) -> Result<String, String> {
    encode(&Header::default(), user, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
        .map_err(|e| e.to_string())
}

impl FromRequest for UserData {
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data = req.headers()
                .get("authorization")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.split("Bearer ").last());
            match data {
                Some(token) => {
                    let state = req.app_data::<AppState>().unwrap();
                    validate_token(token, &state.conn)
                        .await
                        .and_then(|user| user.ok_or("用户找不到".to_string()))
                        .map_err(|e| MyError::Msg(e.to_string()))
                }
                None => Err(MyError::Msg("invalid token".to_string())),
            }
        })
    }
}