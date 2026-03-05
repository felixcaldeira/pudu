use axum::{
    extract::{State, Path, Multipart, multipart::MultipartRejection},
    response::{Response, IntoResponse, Redirect},
};
use serde::Deserialize;
use crate::handlers::AppState;
use crate::AppError;
use crate::models::module_lesson::{ModuleLesson, ModuleLessonUpdate};

pub async fn put(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id)): Path<(String, String, u32)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let mut title = String::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "title" {
            title = field.text().await?;
        }
    }

    let update_data = ModuleLessonUpdate {
        title: Some(title),
        position: None,
    };

    ModuleLesson::update(&state.db, lesson_id, update_data).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/lessons",
        category_slug, module_slug
    )).into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id)): Path<(String, String, u32)>,
) -> Result<Response, AppError> {
    ModuleLesson::delete(&state.db, lesson_id).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/lessons",
        category_slug, module_slug
    )).into_response())
}