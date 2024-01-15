use serde::Deserialize;
use std::io::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>().map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}
