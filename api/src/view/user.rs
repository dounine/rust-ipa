use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json, Path, Query, scope, ServiceConfig};
use serde::Deserialize;
use tracing::{instrument};
use tracing::log::debug;
use entity::user::UserType;
use crate::error::MyError;
use crate::response::{resp_list, resp_ok};
use crate::state::AppState;
use crate::token;
use crate::view::base::PageOptions;

#[get("")]
#[instrument(skip(state))]
async fn user_list(state: Data<AppState>, page: Query<PageOptions>) -> Result<HttpResponse, MyError> {
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
async fn user_detail(state: Data<AppState>, id: Path<i32>) -> Result<HttpResponse, MyError> {
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
    debug!("login data: {} {}",data.username,data.password);
    let token = token::create_token(&token::UserData {
        id: 1,
        user_type: UserType::User,
    }).unwrap();
    Ok(HttpResponse::Ok().json(resp_ok(token)))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(user_list)
            .service(user_detail)
            .service(user_login)
    );
}