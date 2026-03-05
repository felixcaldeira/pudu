use axum::{
    extract::{Request, State, Path, Multipart, multipart::MultipartRejection, FromRequest},
    Json,
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::Image;
use crate::models::module_category::{ModuleCategory};
use crate::models::module::{Module};
use crate::models::module_lesson::{ModuleLesson, ModuleLessonCreate, ModuleLessonUpdate};
use crate::models::module_lesson_section::{ModuleLessonSection};
use crate::models::{User};
use std::collections::HashMap;
use serde::Deserialize;

pub async fn get(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let lessons = ModuleLesson::find_by_module(&state.db, module.id).await?;
        
    let lesson_ids: Vec<u32> = lessons.iter().map(|lesson| lesson.id).collect();
    let sections = ModuleLessonSection::find_by_lesson_ids(&state.db, &lesson_ids).await?;
    let sections_map: HashMap<u32, Vec<ModuleLessonSection>> = sections
        .into_iter()
        .fold(HashMap::new(), |mut map, section| {
            map.entry(section.module_lesson_id)
                .or_insert_with(Vec::new)
                .push(section);
            map
        });

    context.insert("page_title", "Dashboard / Unterrichtseinheiten");
    context.insert("module_category", &module_category);
    context.insert("module", &module);
    context.insert("lessons", &lessons);
    context.insert("sections", &sections_map);

    let html = TERA.render("manage/single_module_lessons.html", &context)?;
    
    Ok(Html(html))
} 

pub async fn post(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let category = ModuleCategory::find_by_slug(&state.db, &category_slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut title = String::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "title" {
            title = field.text().await?;
        }
    }

    let lessons = ModuleLesson::find_by_module(&state.db, module.id).await?;
    let max_position = lessons.iter().map(|l| l.position).max().unwrap_or(0);

    let create_data = ModuleLessonCreate {
        module_id: module.id,
        title,
        position: max_position + 1,
    };

    ModuleLesson::create(&state.db, create_data).await?;

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
    Path((category_slug, module_slug)): Path<(String, String)>,
    Json(payload): Json<ReorderRequest>,
) -> Result<Response, AppError> {
    let updates: Vec<(u32, u32)> = payload.updates
        .iter()
        .map(|u| (u.id, u.position))
        .collect();

    ModuleLesson::batch_update_positions(&state.db, &updates).await?;

    Ok(().into_response())
}