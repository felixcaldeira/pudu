use axum::{
    extract::{State, Path, Json, Multipart, multipart::MultipartRejection},
    response::{Response, IntoResponse, Redirect},
};
use serde::Deserialize;
use crate::handlers::AppState;
use crate::AppError;
use crate::models::module_lesson::ModuleLesson;
use crate::models::module_lesson_section::{ModuleLessonSection, ModuleLessonSectionCreate, ModuleLessonSectionUpdate};

pub async fn post(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id)): Path<(String, String, u32)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let lesson = ModuleLesson::find_by_id(&state.db, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;
    
    let mut title = String::new();
    let mut duration: Option<u32> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                title = field.text().await?;
            }
            "duration" => {
                let text = field.text().await?;
                duration = text.parse().ok();
            }
            _ => {}
        }
    }

    let sections = ModuleLessonSection::find_by_lesson(&state.db, lesson_id).await?;
    let max_position = sections.iter().map(|s| s.position).max().unwrap_or(0);

    let create_data = ModuleLessonSectionCreate {
        module_lesson_id: lesson_id,
        title,
        content: None,
        duration: duration.unwrap_or(0),
        position: max_position + 1,
    };

    ModuleLessonSection::create(&state.db, create_data).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/lessons",
        category_slug, module_slug
    )).into_response())
}

#[derive(Deserialize)]
pub struct PositionUpdate {
    pub id: u32,
    pub position: u32,
}

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub updates: Vec<PositionUpdate>,
}

pub async fn reorder(
    State(state): State<AppState>,
    Path((category_slug, module_slug, lesson_id)): Path<(String, String, u32)>,
    Json(payload): Json<ReorderRequest>,
) -> Result<Response, AppError> {
    let updates: Vec<(u32, u32)> = payload.updates
        .iter()
        .map(|u| (u.id, u.position))
        .collect();

    ModuleLessonSection::batch_update_positions(&state.db, &updates).await?;

    Ok(().into_response())
}