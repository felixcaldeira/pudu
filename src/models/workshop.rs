// src/models/workshop.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::handlers::error::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Workshop {
    pub id: u32,
    pub image_id: Option<u32>,
    
    pub slug: String,
    pub title: String,
    pub description: String,
    pub content: Option<String>,
    pub workshop_date: NaiveDateTime,

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
// todo