use crate::base::config::Config;
use migration::sea_orm::{Database, DatabaseConnection};
use service::sea_orm::ConnectOptions;
use std::sync::Mutex;
use std::time::Duration;
use tracing::log;

pub struct AppState {
    pub users: Mutex<Vec<String>>,
    pub conn: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let config = Config::from_env().unwrap();
        let mut opt = ConnectOptions::new(config.database_url);
        opt.max_connections(5)
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Debug)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));
        let conn = Database::connect(opt)
            .await
            .expect("Cannot connect to database");
        Self {
            users: Mutex::new(vec![]),
            conn,
        }
    }
}
