// src/models/article.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::handlers::error::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: u32,
    pub image_id: Option<u32>,
    
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}