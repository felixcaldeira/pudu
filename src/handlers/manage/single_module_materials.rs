use axum::{
    extract::{State, Path, Json, Multipart, multipart::MultipartRejection},
    response::{Html, Response, IntoResponse, Redirect},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::module::Module;
use crate::models::module_category::ModuleCategory;
use crate::models::module_material::{ModuleMaterial, ModuleMaterialCreate, ModuleMaterialUpdate};
use crate::models::File;
use std::collections::HashMap;
use axum::extract::Request;
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

    let materials = ModuleMaterial::find_by_module(&state.db, module.id).await?;

    let file_ids: Vec<u32> = materials.iter().map(|m| m.file_id).collect();
    let mut files_map = HashMap::new();
    
    for file_id in file_ids {
        if let Some(file) = File::find_by_id(&state.db, file_id).await? {
            files_map.insert(file_id, file);
        }
    }

    context.insert("page_title", &format!("{} - Materialien", module.title));
    context.insert("module_category", &module_category);
    context.insert("module", &module);
    context.insert("materials", &materials);
    context.insert("files", &files_map);

    let html = TERA.render("manage/single_module_materials.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;

    let module = Module::find_by_slug(&state.db, &module_slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut title = String::new();
    let mut material_type = String::new();
    let mut file_id: Option<u32> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                title = field.text().await?;
            }
            "material_type" => {
                material_type = field.text().await?;
            }
            "file" => {
                if let Some(file_name) = field.file_name() {
                    let file_name = file_name.to_string();
                    if let Some(content_type) = field.content_type() {
                        let mime_type = content_type.to_string();
                        let bytes = field.bytes().await?;
                        
                        if !bytes.is_empty() {
                            let file = File::create(&state.db, &bytes, &file_name, &mime_type).await?;
                            file_id = Some(file.id);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let file_id = file_id.ok_or(AppError::BadRequest("File is required".into()))?;

    let materials = ModuleMaterial::find_by_module(&state.db, module.id).await?;
    let max_position = materials.iter().map(|m| m.position).max().unwrap_or(0);

    let create_data = ModuleMaterialCreate {
        module_id: module.id,
        file_id,
        title,
        material_type,
        position: max_position + 1,
    };

    ModuleMaterial::create(&state.db, create_data).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/materials",
        category_slug, module_slug
    )).into_response())
}

pub async fn put(
    State(state): State<AppState>,
    Path((category_slug, module_slug, material_id)): Path<(String, String, u32)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let mut title: Option<String> = None;
    let mut material_type: Option<String> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    title = Some(text);
                }
            }
            "material_type" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    material_type = Some(text);
                }
            }
            _ => {}
        }
    }

    let update_data = ModuleMaterialUpdate {
        title,
        material_type,
        position: None,
    };

    ModuleMaterial::update(&state.db, material_id, update_data).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/materials",
        category_slug, module_slug
    )).into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    Path((category_slug, module_slug, material_id)): Path<(String, String, u32)>,
) -> Result<Response, AppError> {
    ModuleMaterial::delete(&state.db, material_id).await?;

    Ok(Redirect::to(&format!(
        "/manage/modules/{}/{}/materials",
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

    ModuleMaterial::batch_update_positions(&state.db, &updates).await?;

    Ok(().into_response())
}