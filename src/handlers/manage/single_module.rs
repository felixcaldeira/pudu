use axum::{
    extract::{Request, State, Path, Multipart, multipart::MultipartRejection, FromRequest},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::{User, Filters, Module, ModuleCategory, Image, GradeFlags};
use crate::models::module::{ModuleUpdate};

pub async fn get(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let module_categories = ModuleCategory::find_all(&state.db, Filters {page: Some(1), limit: None, order: Some("created_at".to_string()), descending:Some(false), published: None}).await?;
    let users = User::find_all(&state.db, Filters {page: Some(1), limit: None, order: Some("first_name".to_string()), descending:Some(false), published: None}).await?;

    let mut image: Option<Image> = None;

    if let Some(image_id) = module.image_id {
        image = Image::find_by_id(&state.db, image_id).await?;
    }
        
    context.insert("page_title", "Dashboard / Unterrichtseinheiten");
    context.insert("module_category", &module_category);
    context.insert("module_categories", &module_categories);
    context.insert("users", &users);
    context.insert("module", &module);
    context.insert("image", &image);

    let html = TERA.render("manage/single_module.html", &context)?;
    
    Ok(Html(html))
}

pub async fn put(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut content: Option<String> = None;
    let mut published: Option<i8> = None;
    let mut image_id: Option<u32> = None;
    let mut category_id: Option<u32> = None;
    let mut user_id: Option<u32> = None;
    let mut grade_flags: GradeFlags = GradeFlags::UNSET;

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
            "content" => {
                let text = field.text().await?;
                if !text.is_empty() {
                    content = Some(text);
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
                        
                        if let Some(old_image_id) = module.image_id {
                            Image::delete(&state.db, old_image_id).await?;
                        }
                    }
                }
            }
            "category_id" => {
                let text = field.text().await?;
                if !text.trim().is_empty() {
                    category_id = text.trim().parse().ok();
                }
            }
            "user_id" => {
                let text = field.text().await?;
                if !text.trim().is_empty() {
                    user_id = text.trim().parse().ok();
                }
            }
            "grade_flags" => {
                let text = field.text().await?;
                if !text.trim().is_empty() {
                    grade_flags = GradeFlags::from_bits_truncate(text.trim().parse().unwrap_or(0));
                }
            }
            _ => {}
        }
    }

    let category_id = category_id.ok_or(AppError::BadRequest("Kategorie ist erforderlich".into()))?;

    // if let Some(cat_id) = category_id {
    let category = ModuleCategory::find_by_id(&state.db, category_id, None)
        .await?
        .ok_or(AppError::NotFound)?;
    // }

    if let Some(uid) = user_id {
        User::find_by_id(&state.db, uid)
            .await?
            .ok_or(AppError::NotFound)?;
    }

    let update_data = ModuleUpdate {
        title,
        description,
        content,
        category_id: Some(category_id),
        image_id,
        user_id,
        grade_flags: Some(grade_flags),
        published,
    };

    Module::update(&state.db, module.id, update_data).await?;

    Ok(Redirect::to(&format!("/manage/modules/{}/{}", category.slug, module_slug).to_string()).into_response())
}

pub async fn delete(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, None)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(image_id) = module.image_id {
        Image::delete(&state.db, image_id).await?;
    }
    Module::delete(&state.db, module.id).await?;

    Ok(Redirect::to(&format!("/manage/modules/{}", module_category.slug).to_string()).into_response())
}