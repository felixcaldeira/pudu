// src/models/image.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::handlers::error::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: u32,
    pub nanoid: String, // unique nanoid
    pub mimetype: String,
    pub created_at: Option<NaiveDateTime>,
}