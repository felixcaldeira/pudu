use axum::{
    extract::{Request, State, Path, Query},
    response::{Html},
};
use tera::Context;
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::{Module, ModuleCategory, ModuleLesson, ModuleLessonSection, ModuleMaterial, File, Image, Filters, GradeFlags, User};
use std::collections::HashMap;

pub async fn get(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let mut image: Option<Image> = None;
    let mut author: Option<User> = None;

    if let Some(image_id) = module.image_id {
        image = Image::find_by_id(&state.db, image_id).await?;
    }
    if let Some(user_id) = module.user_id {
        author = User::find_by_id(&state.db, user_id).await?;
    }

    let grades: Vec<&'static str> = module.grade_flags.to_strings();

    context.insert("page_title", &module.title);
    context.insert("module_category", &module_category);
    context.insert("module", &module);
    context.insert("image", &image);
    context.insert("user", &author);
    context.insert("grades", &grades);

    let html = TERA.render("single_module.html", &context)?;
    
    Ok(Html(html))
}
pub async fn get_lessons(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let mut image: Option<Image> = None;
    let mut author: Option<User> = None;

    if let Some(image_id) = module.image_id {
        image = Image::find_by_id(&state.db, image_id).await?;
    }
    if let Some(user_id) = module.user_id {
        author = User::find_by_id(&state.db, user_id).await?;
    }

    let grades: Vec<&'static str> = module.grade_flags.to_strings();

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

    context.insert("page_title", &module.title);
    context.insert("module_category", &module_category);
    context.insert("module", &module);
    context.insert("image", &image);
    context.insert("user", &author);
    context.insert("grades", &grades);
    context.insert("lessons", &lessons);
    context.insert("sections", &sections_map);

    let html = TERA.render("single_module_lessons.html", &context)?;
    
    Ok(Html(html))
}
pub async fn get_materials(
    State(state): State<AppState>,
    Path((category_slug, module_slug)): Path<(String, String)>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let module = Module::find_by_slug(&state.db, &module_slug, Some(true))
        .await?
        .ok_or(AppError::NotFound)?;

    let mut image: Option<Image> = None;
    let mut author: Option<User> = None;

    if let Some(image_id) = module.image_id {
        image = Image::find_by_id(&state.db, image_id).await?;
    }
    if let Some(user_id) = module.user_id {
        author = User::find_by_id(&state.db, user_id).await?;
    }

    let grades: Vec<&'static str> = module.grade_flags.to_strings();

    let materials = ModuleMaterial::find_by_module(&state.db, module.id).await?;

    let file_ids: Vec<u32> = materials.iter().map(|m| m.file_id).collect();
    let mut files_map = HashMap::new();
    
    for file_id in file_ids {
        if let Some(file) = File::find_by_id(&state.db, file_id).await? {
            files_map.insert(file_id, file);
        }
    }

    context.insert("page_title", &module.title);
    context.insert("module_category", &module_category);
    context.insert("module", &module);
    context.insert("image", &image);
    context.insert("user", &author);
    context.insert("grades", &grades);
    context.insert("materials", &materials);
    context.insert("files", &files_map);

    let html = TERA.render("single_module_materials.html", &context)?;
    
    Ok(Html(html))
}
