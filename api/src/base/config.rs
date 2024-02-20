use crate::base::error::ApiError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,

    pub wechat_app_name: String,
    pub wechat_referrer: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ApiError> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>().map_err(|e| ApiError::msg(e.to_string()))
    }
}
