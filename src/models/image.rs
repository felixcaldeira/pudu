// src/models/image.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;
use nanoid::nanoid;
use std::env;
use tokio::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: u32,
    pub nanoid: String, // unique nanoid
    pub mime_type: String,
    pub ext: String,
    pub created_at: Option<NaiveDateTime>,
}

impl Image {
    fn generate_nanoid() -> String {
        use nanoid::nanoid;
        nanoid!(12)
    }

    fn get_extension_from_mime_type(mime_type: &str) -> Result<&str, AppError> {
        match mime_type {
            "image/jpeg" | "image/jpg" => Ok("jpg"),
            "image/png" => Ok("png"),
            "image/gif" => Ok("gif"),
            "image/webp" => Ok("webp"),
            "image/svg+xml" => Ok("svg"),
            "image/bmp" => Ok("bmp"),
            "image/tiff" => Ok("tiff"),
            _ => Err(AppError::BadRequest(format!("Unsupported image type: {}", mime_type))),
        }
    }

    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Image,
                "SELECT * FROM images WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }
    
    pub async fn find_by_ids(
        pool: &sqlx::MySqlPool,
        ids: &[u32],
    ) -> Result<Vec<Self>, AppError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = ids.iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!("SELECT * FROM images WHERE id IN ({})", placeholders);

        let mut query_builder = sqlx::query_as::<_, Image>(&query);
        
        for id in ids {
            query_builder = query_builder.bind(id);
        }

        Ok(query_builder.fetch_all(pool).await?)
    }

    pub async fn find_by_nanoid(
        pool: &sqlx::MySqlPool,
        nanoid: String,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Image,
                "SELECT * FROM images WHERE nanoid = ?",
                nanoid
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        bytes: &[u8],
        mime_type: &str,
    ) -> Result<Self, AppError> {
        let ext = Self::get_extension_from_mime_type(mime_type)?;
        let nanoid = Self::generate_nanoid();
        
        let file_name = format!("{}.{}", nanoid, ext);
        
        let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string());
        fs::create_dir_all(&images_dir).await?;

        let file_path = Path::new(&images_dir).join(&file_name);
        fs::write(&file_path, bytes).await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO images (nanoid, mime_type, ext)
            VALUES (?, ?, ?)
            "#,
            nanoid,
            mime_type,
            ext
        )
        .execute(pool)
        .await?;

        let id = result.last_insert_id() as u32;

        Image::find_by_id(pool, id)
            .await?
            .ok_or(AppError::Internal("Failed to load created image".into()))
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        let image = Image::find_by_id(pool, id).await?
            .ok_or(AppError::NotFound)?;
        
        let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string());
        
        let file_name = format!("{}.{}", image.nanoid, image.ext);
        let file_path = Path::new(&images_dir).join(&file_name);

        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }

        sqlx::query!(
            "DELETE FROM images WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_by_nanoid(
        pool: &sqlx::MySqlPool,
        nanoid: String,
    ) -> Result<(), AppError> {
        let image = Image::find_by_nanoid(pool, nanoid).await?
            .ok_or(AppError::NotFound)?;
        
        Self::delete(pool, image.id).await
    }

    pub fn get_file_path(&self) -> Result<std::path::PathBuf, AppError> {
        let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| "./images".to_string());
        let file_name = format!("{}.{}", self.nanoid, self.ext);
        Ok(Path::new(&images_dir).join(file_name))
    }

    pub async fn read_bytes(&self) -> Result<Vec<u8>, AppError> {
        let file_path = self.get_file_path()?;
        Ok(fs::read(&file_path).await?)
    }
}