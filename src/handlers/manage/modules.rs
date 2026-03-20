use axum::{
    extract::{Request, State, Path, Multipart, Query, FromRequest},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::{Image, ModuleCategory, Filters, Module, User, GradeFlags};
use crate::models::module::ModuleCreate;
use std::collections::HashMap;

pub async fn get(
    State(state): State<AppState>,
    Path(category_slug): Path<String>,
    Query(filters): Query<Filters>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, None)
    .await?
    .ok_or(AppError::NotFound)?;
    let module_categories = ModuleCategory::find_all(&state.db, Filters {page: Some(1), limit: Some(100), order: Some("created_at".to_string()), descending:Some(false), published: None}).await?;
    let modules = Module::find_by_category(&state.db, module_category.id, filters).await?;

    let image_ids: Vec<u32> = modules
        .iter()
        .filter_map(|cat| cat.image_id)
        .collect();

    let images = Image::find_by_ids(&state.db, &image_ids).await?;

    let images_map: HashMap<u32, Image> = images
        .into_iter()
        .map(|img| (img.id, img))
        .collect();
        
    context.insert("page_title", "Dashboard / Unterrichtseinheiten");
    context.insert("module_category", &module_category);
    context.insert("module_categories", &module_categories);
    context.insert("modules", &modules);
    context.insert("images", &images_map);

    let html = TERA.render("manage/modules.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    request: Request,
) -> Result<Response, AppError> {
    let user = request.extensions().get::<User>().cloned();

    let mut multipart = Multipart::from_request(request, &state).await?;

    let mut title = String::new();
    let mut description = String::new();
    let mut content = String::new();
    let mut published: Option<i8> = None;
    let mut image_id: Option<u32> = None;
    let mut category_id: Option<u32> = None;
    let mut grade_flags: GradeFlags = GradeFlags::UNSET; // Default to 0, not Option

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
            "category_id" => {
                let text = field.text().await?;
                if !text.trim().is_empty() {
                    category_id = text.trim().parse().ok();
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

    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    let create_data = ModuleCreate {
        image_id,
        user_id: user.map(|u| u.id),
        category_id: Some(category_id),
        slug: slug.clone(),
        title,
        description,
        content: Some(content),
        grade_flags,
        published,
    };

    Module::create(&state.db, create_data).await?;

    Ok(Redirect::to(&format!("/manage/modules/{}/{}", category.slug, slug).to_string()).into_response())
}