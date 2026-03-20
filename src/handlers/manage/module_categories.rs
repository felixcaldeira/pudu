use axum::{
    extract::{Request, State, Path, Multipart, Query, multipart::MultipartRejection},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::{Image, Filters, ModuleCategory};
use crate::models::module_category::{ModuleCategoryCreate, ModuleCategoryUpdate};
use std::collections::HashMap;

pub async fn get(
    State(state): State<AppState>,
    Query(filters): Query<Filters>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_categories = ModuleCategory::find_all(&state.db, filters).await?;

    let image_ids: Vec<u32> = module_categories
        .iter()
        .filter_map(|cat| cat.image_id)
        .collect();

    let images = Image::find_by_ids(&state.db, &image_ids).await?;

    let images_map: HashMap<u32, Image> = images
        .into_iter()
        .map(|img| (img.id, img))
        .collect();
        
    context.insert("page_title", "Dashboard / Fachgruppen");
    context.insert("module_categories", &module_categories);
    context.insert("images", &images_map);

    let html = TERA.render("manage/module_categories.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let mut title = String::new();
    let mut description = String::new();
    let mut published: Option<i8> = None;
    let mut image_id: Option<u32> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                title = field.text().await?;
            }
            "description" => {
                description = field.text().await?;
            }
            "published" => {
                published = Some(1);
            }
            "image" => {
                if let Some(content_type) = field.content_type() {
                    let mime_type = content_type.to_string();
                    
                    let bytes = field.bytes().await?;
                    
                    if !bytes.is_empty() {
                        let image = Image::create(&state.db, &bytes, &mime_type).await?;
                        image_id = Some(image.id);
                    }
                }
            }
            _ => {}
        }
    }

    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    let create_data = ModuleCategoryCreate {
        image_id,
        slug: slug.clone(),
        title,
        description,
        published,
    };

    ModuleCategory::create(&state.db, create_data).await?;

    Ok(Redirect::to(&format!("/manage/modules/{}", slug).to_string()).into_response())
}

pub async fn put(
    State(state): State<AppState>,
    Path(category_slug): Path<String>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;
    let category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut published: Option<i8> = None;
    let mut image_id: Option<u32> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "title" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    title = Some(text);
                }
            }
            "description" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    description = Some(text);
                }
            }
            "published" => {
                published = Some(1);
            }
            "image" => {
                if let Some(content_type) = field.content_type() {
                    let mime_type = content_type.to_string();
                    let bytes = field.bytes().await?;
                    
                    if !bytes.is_empty() {
                        let image = Image::create(&state.db, &bytes, &mime_type).await?;
                        image_id = Some(image.id);
                        
                        if let Some(old_image_id) = category.image_id {
                            Image::delete(&state.db, old_image_id).await?;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let update_data = ModuleCategoryUpdate {
        title,
        description,
        image_id,
        published,
    };

    ModuleCategory::update(&state.db, category.id, update_data).await?;

    Ok(Redirect::to("/manage/modules").into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    Path(category_slug): Path<String>,
) -> Result<Response, AppError> {
    let category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;
    if let Some(image_id) = category.image_id {
        Image::delete(&state.db, image_id).await?;
    }
    ModuleCategory::delete(&state.db, category.id).await?;

    Ok(Redirect::to("/manage/modules").into_response())
}