// src/models/file.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use crate::AppError;
use sqlx::FromRow;
use std::env;
use tokio::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: u32,
    pub file_name: String,
    pub mime_type: String,
    pub created_at: Option<NaiveDateTime>,
}

impl File {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                File,
                "SELECT * FROM files WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_file_name(
        pool: &sqlx::MySqlPool,
        file_name: String,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                File,
                "SELECT * FROM files WHERE file_name = ?",
                file_name
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        bytes: &[u8],
        original_file_name: &str,
        mime_type: &str,
    ) -> Result<Self, AppError> {
        let _ext = Path::new(original_file_name)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(AppError::BadRequest("File must have an extension".into()))?;
        let date_prefix = Utc::now().naive_utc().format("%Y%m%d").to_string();
        let file_name = format!("{}_{}", date_prefix, original_file_name);
        
        let files_dir: String = env::var("FILES_DIR").unwrap_or_else(|_| "./files".to_string());
        fs::create_dir_all(&files_dir).await?;

        let file_path = Path::new(&files_dir).join(&file_name);
        fs::write(&file_path, bytes).await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO files (file_name, mime_type)
            VALUES (?, ?)
            "#,
            file_name,
            mime_type
        )
        .execute(pool)
        .await?;


        let id = result.last_insert_id() as u32;

        File::find_by_id(pool, id)
            .await?
            .ok_or(AppError::Internal("Failed to load created file".into()))
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        let file = File::find_by_id(pool, id).await?;
        
        let files_dir: String = env::var("FILES_DIR").unwrap_or_else(|_| "./files".to_string());
        let file_path = Path::new(&files_dir).join(&file.unwrap().file_name);

        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }

        sqlx::query!(
            "DELETE FROM files WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}