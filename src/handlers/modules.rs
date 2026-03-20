use axum::{
    extract::{Request, State, Path, Query},
    response::{Html},
};
use tera::Context;
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::{Module, ModuleCategory, Image, Filters, GradeFlags, User};
use std::collections::HashMap;

static LIMIT: u32 = 10;

pub async fn get_all(
    State(state): State<AppState>,
    Query(filters): Query<Filters>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);
    
    let categories = ModuleCategory::find_all(&state.db, Filters {page: filters.page, limit: Some(LIMIT), order: Some(filters.order.unwrap_or("title".to_string())), descending: filters.descending, published: Some(true)}).await?;
    let page: u32 = filters.page.unwrap_or(1);
    let category_count = ModuleCategory::count(&state.db, Some(true)).await?;
    let total_pages = (category_count as f64 / LIMIT as f64).ceil() as u32;

    let image_ids: Vec<u32> = categories
        .iter()
        .filter_map(|cat| cat.image_id)
        .collect();
    let images = Image::find_by_ids(&state.db, &image_ids).await?;
    let images_map: HashMap<u32, Image> = images
        .into_iter()
        .map(|img| (img.id, img))
        .collect(); 

    context.insert("page_title", "Fachgruppen / Unterrichtseinheiten");
    context.insert("module_categories", &categories);
    context.insert("current_page", &page);
    context.insert("total_pages", &total_pages);
    context.insert("images", &images_map);

    let html = TERA.render("module_categories.html", &context)?;
    
    Ok(Html(html))
}

pub async fn get(
    State(state): State<AppState>,
    Path(category_slug): Path<String>,
    Query(filters): Query<Filters>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let modules = Module::find_by_category(&state.db, module_category.id, Filters {page: filters.page, limit: filters.limit, order: filters.order, descending: filters.descending, published: Some(true)}).await?;
    let page: u32 = filters.page.unwrap_or(1);
    let module_count = Module::count_by_category(&state.db, module_category.id, Some(true)).await?;
    let total_pages = (module_count as f64 / LIMIT as f64).ceil() as u32;

    let image_ids: Vec<u32> = modules
        .iter()
        .filter_map(|cat| cat.image_id)
        .collect();
    let images = Image::find_by_ids(&state.db, &image_ids).await?;
    let images_map: HashMap<u32, Image> = images
        .into_iter()
        .map(|img| (img.id, img))
        .collect();

    let author_ids: Vec<u32> = modules
        .iter()
        .filter_map(|cat| cat.user_id)
        .collect();
    let authors = User::find_by_ids(&state.db, &author_ids).await?;
    let authors_map: HashMap<u32, User> = authors
        .into_iter()
        .map(|usr| (usr.id, usr))
        .collect();

    let grades_map: HashMap<u32, Vec<&'static str>> = modules.clone()
        .into_iter()
        .map(|m| (m.id, m.grade_flags.to_strings()))
        .collect();

    context.insert("page_title", &format!("{} / Unterrichtseinheiten", module_category.title));
    context.insert("module_category", &module_category);
    context.insert("modules", &modules);
    context.insert("current_page", &page);
    context.insert("total_pages", &total_pages);
    context.insert("images", &images_map);
    context.insert("authors", &authors_map);
    context.insert("grades", &grades_map);
    
    let html = TERA.render("modules.html", &context)?;
    
    Ok(Html(html))
}
