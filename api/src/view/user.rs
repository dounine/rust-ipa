use crate::error::MyError;
use crate::response::{resp_list, resp_ok};
use crate::state::AppState;
use crate::token;
use crate::token::UserData;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::user::{Model, UserStatus, UserType};
use serde::Deserialize;
use tracing::instrument;
use tracing::log::debug;

#[get("")]
#[instrument(skip(state))]
async fn user_list(
    state: Data<AppState>,
    page: Query<PageOptions>,
) -> Result<HttpResponse, MyError> {
    debug!("进去store查询数据中...");
    let page = page.format();
    service::user::list_user(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| resp_list(l, total))
        .map(|users| HttpResponse::Ok().json(users))
        .map(Ok)?
}

#[get("/{id}")]
#[instrument(skip(state))]
async fn user_detail(
    state: Data<AppState>,
    user: UserData,
    id: Path<i32>,
) -> Result<HttpResponse, MyError> {
    service::user::find_user_by_id(&state.conn, id.into_inner())
        .await
        .map(|user| resp_ok(user))
        .map(|user| HttpResponse::Ok().json(user))
        .map(Ok)?
}

#[derive(Deserialize, Debug)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
#[instrument(skip(state))]
async fn user_login(state: Data<AppState>, data: Json<LoginData>) -> Result<HttpResponse, MyError> {
    debug!("login data: {} {}", data.username, data.password);
    let user_query = if data.username.contains("@") {
        service::user::find_user_by_email(&state.conn, data.username.as_str()).await
    } else {
        service::user::find_user_by_username(&state.conn, data.username.as_str()).await
    };
    user_query.map(|user| match user {
        Some(result) => {
            if result.status == UserStatus::Disable {
                return Err(MyError::Msg("用户已被禁用".to_string()));
            }
            if result.password.is_none() {
                return Err(MyError::Msg("帐号或者密码错误".to_string()));
            }
            let hash = md5::compute(data.password.as_bytes());
            match format!("{:?}", hash) == result.password.unwrap_or_default() {
                true => {
                    let token = token::create_token(1, UserType::User, 30).unwrap();
                    Ok(HttpResponse::Ok().json(resp_ok(token)))
                }
                false => Err(MyError::Msg("帐号或者密码错误".to_string())),
            }
        }
        None => Err(MyError::Msg("用户不存在".to_string())),
    })?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(user_list)
            .service(user_detail)
            .service(user_login),
    );
}
