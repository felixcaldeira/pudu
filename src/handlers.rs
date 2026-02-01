use sqlx::MySqlPool;
use tera::Tera;
use crate::config::Config;

pub mod home;
pub mod error;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
}