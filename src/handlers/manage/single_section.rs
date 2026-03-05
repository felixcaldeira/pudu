use axum::{
    extract::{State, Path, Json, Multipart, multipart::MultipartRejection},
    response::{Response, IntoResponse, Redirect},
};
use serde::Deserialize;
use crate::handlers::AppState;
use crate::AppError;
use crate::models::module_lesson_section::{ModuleLessonSection, ModuleLessonSectionUpdate};

#[derive(Deserialize)]
pub struct UpdateSectionForm {
    pub title: String,
    pub content: String,
    pub duration: Option<u32>,
}

pub async fn put(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id, section_id)): Path<(String, String, u32, u32)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;

    let mut title = String::new();
    let mut content = String::new();
    let mut duration: Option<u32> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                title = field.text().await?;
            }
            "content" => {
                content = field.text().await?;
            }
            "duration" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    duration = text.parse().ok();
                }
            }
            _ => {}
        }
    }

    let update_data = ModuleLessonSectionUpdate {
        title: Some(title),
        content: Some(content),
        duration,
        position: None,
    };

    ModuleLessonSection::update(&state.db, section_id, update_data).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/lessons",
        category_slug, module_slug
    )).into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id, section_id)): Path<(String, String, u32, u32)>,
) -> Result<Response, AppError> {
    ModuleLessonSection::delete(&state.db, section_id).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/lessons",
        category_slug, module_slug
    )).into_response())
}