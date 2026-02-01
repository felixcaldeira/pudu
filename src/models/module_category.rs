use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::handlers::error::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleCategory {
    pub id: u32,
    pub image_id: Option<u32>, // banner bild

    pub slug: String, // seo-freundliche-domain
    pub title: String,
    pub description: String,

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}