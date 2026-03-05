use sqlx::MySqlPool;
use tera::Tera;
use crate::config::Config;

pub mod home;
pub mod error;
pub mod login;
pub mod register;
pub mod manage;
pub mod contact;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
}