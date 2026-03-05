use serde::Deserialize;
use std::env;
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: IpAddr,
    pub port: u16,
    pub domain: String,
    pub files_dir: String,
    pub images_dir: String,
    pub static_dir: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            smtp_host: env::var("SMTP_HOST")?,
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()?,
            smtp_user: env::var("SMTP_USER")?,
            smtp_pass: env::var("SMTP_PASS")?,
            host: env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string())
                .parse()?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3003".to_string())
                .parse()?,
            domain: env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
            files_dir: env::var("FILES_DIR").unwrap_or_else(|_| "./files".to_string()),
            images_dir: env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string()),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string()),
        })
    }
}