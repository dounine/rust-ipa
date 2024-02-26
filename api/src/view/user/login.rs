use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use tracing::instrument;
use tracing::log::debug;

use entity::user::{UserStatus, UserType};

use crate::base::error::ApiError;
use crate::base::response::{resp_ok};
use crate::base::state::AppState;
use crate::base::token;

#[derive(Deserialize, Debug)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
#[instrument(skip(state))]
async fn user_login(
    state: Data<AppState>,
    data: Json<LoginData>,
) -> Result<HttpResponse, ApiError> {
    debug!("login data: {} {}", data.username, data.password);
    let user_query = if data.username.contains("@") {
        service::user::find_by_email::find_by_email(&state.conn, data.username.as_str()).await
    } else {
        service::user::find_by_username::find_by_username(&state.conn, data.username.as_str()).await
    };
    user_query.map(|user| match user {
        Some(result) => {
            if result.status == UserStatus::Disable {
                return ApiError::msg("用户已被禁用").into();
            }
            if result.password.is_none() {
                return ApiError::msg("帐号或者密码错误").into();
            }

            match util::crypto::md5(data.password.as_str()) == result.password.unwrap_or_default() {
                true => {
                    let token = token::create_token(1, UserType::User, 30).unwrap();
                    Ok(resp_ok(token).into())
                }
                false => ApiError::msg("帐号或者密码错误").into(),
            }
        }
        None => ApiError::msg("用户不存在").into(),
    })?
}
