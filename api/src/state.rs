use std::env;
use std::sync::Mutex;
use std::time::Duration;
use tracing::log;
use migration::sea_orm::{Database, DatabaseConnection};
use service::sea_orm::ConnectOptions;

pub struct AppState {
    pub users: Mutex<Vec<String>>,
    pub conn: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(5)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);
        let conn = Database::connect(opt).await.expect("Cannot connect to database");
        Self {
            users: Mutex::new(vec![]),
            conn,
        }
    }
}
