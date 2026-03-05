// src/models/newsletter.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Newsletter {
    pub id: u32,
    pub image_id: Option<u32>,

    pub title: String,
    pub description: String,
    pub content: Option<String>,
    
    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
// todo