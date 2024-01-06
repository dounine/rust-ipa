use std::env;
use std::sync::Mutex;
use migration::sea_orm::{Database, DatabaseConnection};

pub struct AppState {
    pub users: Mutex<Vec<String>>,
    pub pool: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
            pool: Database::connect(&env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")).await.unwrap(),
        }
    }
}
