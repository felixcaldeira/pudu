use serde::Deserialize;
use std::env;
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: IpAddr,
    pub port: u16,
    pub files_dir: String,
    pub images_dir: String,
    pub static_dir: String,
    // pub email_host: String,
    // pub email_port: String,
    // pub email_user: String,
    // pub email_pass: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            // email_host: env::var("EMAIL_HOST")?,
            // email_port: env::var("EMAIL_PORT")?,
            // email_user: env::var("EMAIL_USER")?,
            // email_pass: env::var("EMAIL_PASS")?,
            host: env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string())
                .parse()?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3003".to_string())
                .parse()?,
            files_dir: env::var("FILES_DIR").unwrap_or_else(|_| "./files".to_string()),
            images_dir: env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string()),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string()),
        })
    }
}