// src/handlers/image.rs
// use axum::{
//     extract::{Multipart, Path, State},
//     http::{header, StatusCode},
//     response::{IntoResponse, Response},
//     Json,
// };
// use serde::{Deserialize, Serialize};
// use crate::{
//     models::image::Image,
//     handlers::error::AppError,
// };

// #[derive(Debug, Serialize)]
// pub struct ImageResponse {
//     pub id: u32,
//     pub nanoid: String,
//     pub mimetype: String,
//     pub url: String,
//     pub created_at: Option<chrono::NaiveDateTime>,
// }

// impl From<Image> for ImageResponse {
//     fn from(image: Image) -> Self {
//         Self {
//             id: image.id,
//             nanoid: image.nanoid.clone(),
//             mimetype: image.mimetype.clone(),
//             url: format!("/api/images/{}", image.nanoid),
//             created_at: image.created_at,
//         }
//     }
// }

/// Upload a new image
/// POST /api/images
// pub async fn upload_image(
//     State(pool): State<sqlx::MySqlPool>,
//     mut multipart: Multipart,
// ) -> Result<Json<ImageResponse>, AppError> {
//     let mut image_bytes: Option<Vec<u8>> = None;
//     let mut mimetype: Option<String> = None;

//     // Process multipart form data
//     while let Some(field) = multipart.next_field().await? {
//         let name = field.name().unwrap_or("").to_string();
        
//         if name == "image" {
//             mimetype = field.content_type().map(|s| s.to_string());
//             image_bytes = Some(field.bytes().await?.to_vec());
//         }
//     }

//     let image_bytes = image_bytes
//         .ok_or(AppError::BadRequest("No image provided".into()))?;
    
//     let mimetype = mimetype
//         .ok_or(AppError::BadRequest("No content type provided".into()))?;

//     // Validate that it's an image mimetype
//     if !mimetype.starts_with("image/") {
//         return Err(AppError::BadRequest("File must be an image".into()));
//     }

//     // Create image
//     let image = Image::create(&pool, &image_bytes, &mimetype).await?;

//     Ok(Json(ImageResponse::from(image)))
// }

/// Get image by nanoid
/// GET /api/images/:nanoid
// pub async fn get_image(
//     State(pool): State<sqlx::MySqlPool>,
//     Path(nanoid): Path<String>,
// ) -> Result<Response, AppError> {
//     let image = Image::find_by_nanoid(&pool, nanoid)
//         .await?
//         .ok_or(AppError::NotFound("Image not found".into()))?;

//     let bytes = image.read_bytes().await?;

//     Ok((
//         StatusCode::OK,
//         [(header::CONTENT_TYPE, image.mimetype.clone())],
//         bytes,
//     )
//         .into_response())
// }

/// Get image metadata by nanoid
/// GET /api/images/:nanoid/meta
// pub async fn get_image_meta(
//     State(pool): State<sqlx::MySqlPool>,
//     Path(nanoid): Path<String>,
// ) -> Result<Json<ImageResponse>, AppError> {
//     let image = Image::find_by_nanoid(&pool, nanoid)
//         .await?
//         .ok_or(AppError::NotFound("Image not found".into()))?;

//     Ok(Json(ImageResponse::from(image)))
// }

/// Delete image by nanoid
/// DELETE /api/images/:nanoid
// pub async fn delete_image(
//     State(pool): State<sqlx::MySqlPool>,
//     Path(nanoid): Path<String>,
// ) -> Result<StatusCode, AppError> {
//     Image::delete_by_nanoid(&pool, nanoid).await?;
//     Ok(StatusCode::NO_CONTENT)
// }